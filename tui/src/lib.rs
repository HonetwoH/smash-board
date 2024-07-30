#[cfg(feature = "inline")]
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

    use config::Base;

    use ratatui::{
        layout::{Constraint, Direction, Layout, Rect},
        prelude::Style,
        style::Stylize,
        text::Text,
        widgets::{
            Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Widget,
        },
    };

    //TODO: add number parsing abilities in this
    pub struct PromptText {
        base: Base,
        field: String,
    }

    pub(crate) enum ShuffleOperation {
        Pop(u8),
        Push(u8),
    }

    impl PromptText {
        // This is very ugly hack please avoid it
        pub fn new(base: Base) -> Self {
            Self {
                base,
                field: String::from("█"),
            }
        }
        pub fn push(&mut self, x: char) -> Option<ShuffleOperation> {
            // TODO: need a constraint on the length at some point
            let cursor = self.field.pop().unwrap();
            self.field.push(x);
            self.field.push(cursor);
            if let Some(n) = x.to_digit(self.base as u32) {
                Some(ShuffleOperation::Push(n as u8))
            } else {
                None
            }
        }
        pub fn pop(&mut self) -> Option<ShuffleOperation> {
            // 3 because the cursor character is 3 byte unicode
            if self.field.len() > 3 {
                let cursor = self.field.pop().unwrap();
                let x = self.field.pop().unwrap();
                self.field.push(cursor);
                if let Some(n) = x.to_digit(self.base as u32) {
                    Some(ShuffleOperation::Pop(n as u8))
                } else {
                    None
                }
            } else {
                None
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
        pub fn new(base: u8) -> Self {
            Self {
                list_selected: Vec::with_capacity(base as usize),
                list_unselected: Vec::from_iter(0..base),
                size: base as u8,
                selected: 0,
            }
        }

        fn status(&self) -> Vec<u8> {
            let mut first = self.list_selected.clone();
            let mut second = self.list_unselected.clone();
            first.append(&mut second);
            first
        }

        fn select(&mut self, n: u8) {
            assert!(self.selected <= self.size);
            let idx = Self::search(&self.list_unselected, n).unwrap();
            let ele = self.list_unselected.remove(idx as usize);
            assert!(ele == n);
            self.selected += 1;
            self.list_selected.push(n);
            assert!(self.selected <= self.size);
            assert!(self.list_selected.len() + self.list_unselected.len() == self.size as usize);
        }

        fn unselect(&mut self, n: u8) {
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
        size: Option<Rect>,
        // TODO: maybe this is not fine ?
        // scroll_bar: (Scrollbar<'a>, ScrollbarState),
    }

    impl<'a> Preview<'a> {
        pub fn new(blobs: Vec<String>) -> Self {
            let mut preview = {
                let no_of_blocks = blobs.len() as u8;
                let order_of_blocks = ShuffleList::new(no_of_blocks);
                // let scroll = Scrollbar::new(ScrollbarOrientation::VerticalRight);
                // TODO: Should wrap this in a smart pointer
                // let scrollstate = ScrollbarState::new(no_of_blocks as usize).position(0);

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
                    size: None,
                    // scroll_bar: (scroll, scrollstate),
                }
            };
            preview.make_blocks();
            preview
        }

        // this function will modify the blocks as neeeded
        // TODO: make this function actually useful with dynamic values
        pub fn make_blocks(&mut self) {
            let mut blocks = Vec::with_capacity(self.no_of_blocks as usize);
            for i in 0..(self.no_of_blocks as usize) {
                blocks.push(
                    Paragraph::new(Text::raw({
                        let lines: Vec<String> = self.raw_buffer[i].clone();
                        lines
                            .into_iter()
                            .take(5)
                            .fold(String::new(), |mut lines, line| {
                                lines.push_str(&line);
                                lines.push('\n');
                                lines
                            })
                    }))
                    .block(Block::new().borders(Borders::all()).title(format!("{}", i))),
                );
            }
            self.blocks = blocks;
        }

        // TODO: call this in render loop
        fn size_changed(&mut self, area: Rect) {
            if let Some(size) = self.size {
                if size != area {
                    // TODO: is this correct
                    self.make_blocks();
                    self.size = Some(area);
                }
            } else {
                self.size = Some(area)
            }
        }

        pub fn select(&mut self, n: u8) {
            if n >= self.no_of_blocks {
                return;
            }
            let block: Paragraph<'_> = self.blocks[n as usize].clone();
            let new_block = block.block(
                Block::default()
                    .borders(Borders::all())
                    .title(format!("{}", n))
                    .border_style(Style::new().green()),
            );
            self.blocks[n as usize] = new_block;
            self.order_of_blocks.select(n)
        }

        pub fn unselect(&mut self, n: u8) {
            if n >= self.no_of_blocks {
                return;
            }
            let block: Paragraph<'_> = self.blocks[n as usize].clone();
            let new_block = block.block(
                Block::default()
                    .borders(Borders::all())
                    .title(format!("{}", n)),
            );
            self.blocks[n as usize] = new_block;
            self.order_of_blocks.unselect(n)
        }

        pub fn yeild_list(self) -> Vec<Vec<String>> {
            let mut out = Vec::with_capacity(self.order_of_blocks.selected as usize);
            for i in self.order_of_blocks.list_selected {
                out.push(self.raw_buffer[i as usize].clone())
            }
            out
        }
    }

    // TODO: is it possible to do this without STATE
    impl<'a> Widget for &Preview<'a> {
        fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
            // HOWTO: render scrollable list
            let outer_block = Block::new().borders(Borders::all()).title("Preview");
            let inner_area = outer_block.inner(area);
            let max =
                (inner_area.height - (2 * self.no_of_blocks as u16)) / self.no_of_blocks as u16;
            let layout: Rc<[Rect]> = {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![Constraint::Max(max); self.no_of_blocks as usize])
                    .split(inner_area)
            };

            let state = self.order_of_blocks.status();
            // eprintln!("{:?}", &state);
            // TODO: will need to reorder this later
            let mut idx = 0;
            for i in state {
                let block = self.blocks[i as usize].clone();
                block.render(layout[idx], buf);
                idx += 1;
            }
            assert!(idx == self.no_of_blocks as usize);
            outer_block.render(area, buf);
        }
    }
}
