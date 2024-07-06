#[cfg(feature = "interactive")]
pub mod interactive;

#[cfg(feature = "inline")]
pub mod inline;

mod widgets {
    // Interactive Select will blocks with some width and if the blob is
    // selected then its border becomes some other color other than white
    // And block itself contains buffer number and how many times the buffer is called

    use ratatui::{
        layout::Rect,
        prelude::Style,
        style::Stylize,
        widgets::{Block, Paragraph},
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
    // TODO: The layouting will handled here too, so get to it
    // And the blocks will be recreated if the size changed
    struct Preview<'a> {
        // obtained from db, split by lines already
        raw_buffer: Vec<Vec<String>>,
        no_of_lines: Vec<usize>,
        // The vector of blocks which are in same order as raw_buffer
        // its copy will be rearranged as needed
        blocks: Vec<Paragraph<'a>>,
        // order of text this will work with selected to determine how many to color
        order_of_blocks: Vec<u8>,
        // total number of selected blobs
        selected: u8,
        // number of blocks
        no_of_blocks: u8,
        // change in size
        size: (bool, Rect),
    }

    impl<'a> Preview<'a> {
        fn new(blobs: Vec<String>, frame_size: Rect) -> Self {
            let no_of_blocks = blobs.len() as u8;
            let order_of_blocks = ShuffleList::new(no_of_blocks);
            let size = (false, frame_size);

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
                selected: 0,
                no_of_blocks,
                size,
            }
        }

        // This should be triggred automatically
        fn change_style_on_select(&mut self, select: usize) {
            let block: Paragraph<'_> = self.blocks[select].clone();
            let new_block = block.block(Block::default().border_style(Style::new().green()));
            self.blocks[select] = new_block;
        }

        // this function will modify the blocks as neeeded
        // TODO: create a test suite for testing different config  of this functions
        fn make_blocks(&mut self) {
            // text are of height 2 hence divide by 2
            let available_lines = self.size.1.height / 2;
            let avaialbe_width = self.size.1.width;
            let border_size = 2;
            let total: usize = self
                .raw_buffer
                .iter()
                .fold(0, |total, (_, lines)| total + lines);
            // complete demand of least lines first and most lines last
            let request_lines = |required: usize| {};
            todo!("Make sub blocks with most optimal size and each block should be of different if needed")
        }

        fn size_changed(&mut self) {
            todo!("Run this before every iteration automatically and make blocks again")
        }
    }
}
