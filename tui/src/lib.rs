#[cfg(feature = "interactive")]
pub mod interactive;

#[cfg(feature = "inline")]
pub mod inline;

mod widgets {
    // Interactive Select will blocks with some width and if the blob is
    // selected then its border becomes some other color other than white
    // And block itself contains buffer number and how many times the buffer is called

    use ratatui::{
        layout::{Position, Rect},
        text::Line,
        widgets::{Block, Paragraph},
    };

    // List of blocks that is will be rearranged as needed
    // TODO: The layouting will handled here too
    // And the blocks will be recreated if the size changed
    struct ShuffleList<'a> {
        // obtained from db
        raw_buffer: Vec<(Vec<String>, usize)>,
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

    impl<'a> ShuffleList<'a> {
        fn new(blobs: Vec<String>, frame_size: Rect) -> Self {
            let no_of_blocks = blobs.len() as u8;
            let order_of_blocks = Vec::from_iter(0..no_of_blocks);
            let size = (false, frame_size);

            let raw_buffer: Vec<(Vec<String>, usize)> = blobs
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

            let blocks: Vec<Paragraph<'a>> = raw_buffer
                .clone()
                .into_iter()
                .map(|(lines, _no_of_lines)| {
                    lines
                        .into_iter()
                        .take(5)
                        .map(|line| Line::from(line))
                        .collect::<Vec<Line>>()
                })
                .enumerate()
                .map(|(id, lines)| {
                    Paragraph::new(lines).block(Block::default().title(format!("{}", id)))
                })
                .collect();

            Self {
                raw_buffer,
                blocks,
                order_of_blocks,
                selected: 0,
                no_of_blocks,
                size,
            }
        }

        // This should be triggred automatically
        fn change_style_on_select(&mut self) {
            todo!("Change borders style on selection")
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

        // shuffle_blobs take input append which is index of blob
        // it will increse the self.selected and bring `append` to end of
        // selected slice of order vector
        fn shuffle_blobs(&mut self, append: usize) {
            todo!("Change order of shuffled blobs and handle the value of selected")
        }

        fn revert_shuffling(&mut self) {
            todo!()
        }

        fn size_changed(&mut self) {
            todo!("Run this before every iteration automatically and make blocks again")
        }
    }
}
