pub mod inline;

#[cfg(feature = "interactive")]
pub mod interactive;

#[cfg(feature = "interactive")]
mod widgets {
    // Interactive Select will blocks with some width and if the blob is
    // selected then its border becomes some other color other than white
    // And block itself contains buffer number and how many times the buffer is called
    // TODO: Tie Prompt Text, Shuffle List and Preview together

    use std::rc::Rc;

    use ratatui::{
        layout::{self, Constraint, Direction, Layout, Margin, Rect},
        prelude::Style,
        style::Stylize,
        symbols::block,
        text::Text,
        widgets::{
            Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Widget,
        },
    };

    pub struct PromptText {
        field: String,
    }

    impl PromptText {
        // This is very ugly hack please avoid it
        pub fn new() -> Self {
            Self {
                field: String::from("█"),
            }
        }
        pub fn push(&mut self, x: char) {
            // TODO: need a constraint on the length at some point
            let cursor = self.field.pop().unwrap();
            self.field.push(x);
            self.field.push(cursor);
        }
        pub fn pop(&mut self) {
            // 3 because the cursor character is 3 byte unicode
            if self.field.len() > 3 {
                let cursor = self.field.pop().unwrap();
                let _ = self.field.pop().unwrap();
                self.field.push(cursor);
            }
        }
        // Return the prompt text for the rendering
        pub fn dump(&self) -> &str {
            &self.field
        }
        // Return input without the cursor symbol
        fn return_input(&self) -> &str {
            &self.field[0..&self.field.len() - 3]
        }
    }

    #[derive(Debug)]
    pub(crate) struct ShuffleList {
        list_selected: Vec<u8>,
        list_unselected: Vec<u8>,
        size: u8,
        selected: u8,
    }

    impl ShuffleList {
        pub fn new(v: u8) -> Self {
            Self {
                list_selected: Vec::with_capacity(v as usize),
                list_unselected: Vec::from_iter(0..v),
                size: v as u8,
                selected: 0,
            }
        }

        fn status(self) -> Vec<u8> {
            let mut first = self.list_selected.clone();
            let mut second = self.list_unselected.clone();
            first.append(&mut second);
            first
        }

        fn shuffle(&mut self, n: u8) {
            assert!(self.selected <= self.size);
            let idx = Self::search(&self.list_unselected, n).unwrap();
            let ele = self.list_unselected.remove(idx as usize);
            assert!(ele == n);
            self.selected += 1;
            self.list_selected.push(n);
            assert!(self.list_selected.len() + self.list_unselected.len() == self.size as usize);
        }

        fn unshuffle(&mut self, n: u8) {
            let idx = Self::search(&self.list_selected, n).unwrap();
            let ele = self.list_selected.remove(idx as usize);
            assert!(ele == n);
            self.selected -= 1;
            // find the appropriate location to insert n so that it turn out sorted
            let iidx = {
                let mut idx = 0;
                for i in self.list_unselected.iter() {
                    if n > *i {
                        idx += 1;
                    }
                }
                idx as usize
            };
            self.list_unselected.insert(iidx, n);
            assert!(self.list_selected.len() + self.list_unselected.len() == self.size as usize);
        }

        // there must be no duplicates
        fn search(hay: &[u8], pin: u8) -> Option<u8> {
            let mut c = 0;
            for i in hay {
                if *i == pin {
                    return Some(c);
                }
                c += 1
            }
            None
        }
    }
    // List of blocks that is will be rearranged as needed
    // TODO: The layouting will be handled here too, so get to it
    // TODO: add scrollbar
    // And the blocks will be recreated if the size changed
    pub(crate) struct Preview<'a> {
        // TEXT
        // obtained from db, split by lines already
        raw_buffer: Vec<Vec<String>>,
        no_of_lines: Vec<usize>,
        // number of blocks
        no_of_blocks: u8,

        // SHUFFLE LIST
        // order of text this will work with selected to determine how many to color
        order_of_blocks: ShuffleList,

        // RENDER
        // The vector of blocks which are in same order as raw_buffer
        // its copy will be rearranged as needed
        blocks: Vec<Paragraph<'a>>,
        // change in size
        size: Rect,
        // TODO: maybe this is not fine ?
        scroll_bar: (Scrollbar<'a>, ScrollbarState),
    }

    impl<'a> Preview<'a> {
        pub fn new(blobs: Vec<String>, frame_size: Rect) -> Self {
            let no_of_blocks = blobs.len() as u8;
            let order_of_blocks = ShuffleList::new(no_of_blocks);
            let size = frame_size;
            let scroll = Scrollbar::new(ScrollbarOrientation::VerticalRight);
            // TODO: Should wrap this in a smart pointer
            let scrollstate = ScrollbarState::new(no_of_blocks as usize).position(0);

            let (raw_buffer, no_of_lines): (Vec<Vec<String>>, Vec<usize>) = blobs
                .into_iter()
                .map(|b| {
                    let lines = b
                        .split('\n')
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();
                    let len = lines.len();
                    (lines, len)
                })
                .collect();
            assert_eq!(raw_buffer.len(), no_of_lines.len());
            assert_eq!(raw_buffer.len(), no_of_blocks as usize);

            Self {
                raw_buffer,
                no_of_lines,
                blocks: Vec::new(),
                order_of_blocks,
                no_of_blocks,
                size,
                scroll_bar: (scroll, scrollstate),
            }
        }

        // This should be triggred automatically
        pub fn change_style_on_select(&mut self, select: usize) {
            let block: Paragraph<'_> = self.blocks[select].clone();
            // TODO: find something better
            let new_block = block.block(
                Block::default()
                    .borders(Borders::all())
                    .title(format!("{}", select))
                    .border_style(Style::new().green()),
            );
            self.blocks[select] = new_block;
        }

        // this function will modify the blocks as neeeded
        // TODO: create a test scaffolding for testing different config  of this functions
        pub fn make_blocks(&mut self) {
            let mut blocks = Vec::with_capacity(self.no_of_blocks as usize);
            for i in 0..(self.no_of_blocks as usize) {
                blocks.push(
                    Paragraph::new(Text::raw({
                        let lines: Vec<String> = self.raw_buffer[i].clone();
                        let three_lines: String = lines.into_iter().take(3).collect();
                        three_lines
                    }))
                    .block(Block::new().borders(Borders::all()).title(format!("{}", i))),
                );
            }
            self.blocks = blocks;
        }

        fn size_changed(&mut self, area: Rect) {
            if !(self.size == area) {
                self.make_blocks();
                self.size = area;
            }
        }
    }

    // TODO: is it possible to do this without STATE
    impl<'a> Widget for Preview<'a> {
        fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
            // HOWTO: render scrollable list
            let outer_block = Block::new().borders(Borders::all()).title("Preview");
            let inner_area = outer_block.inner(area);
            let layout: Rc<[Rect]> = {
                let for_each = 100 / self.no_of_blocks;

                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Percentage(for_each as u16);
                        self.no_of_blocks as usize
                    ])
                    .split(inner_area)
            };

            // TODO: will need to reorder this later
            for idx in self.order_of_blocks.status() {
                let block = self.blocks[idx as usize].clone();
                block.render(layout[idx as usize], buf);
            }
            // for (idx, blk) in self.blocks.clone().into_iter().enumerate() {
            //     blk.render(layout[idx], buf);
            // }
            outer_block.render(area, buf);
        }
    }
}
