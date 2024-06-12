#[cfg(feature = "interactive")]
pub mod interactive;

#[cfg(feature = "inline")]
pub mod inline;

mod widgets {
    // Interactive Select will blocks with some width and if the blob is
    // selected then its border becomes some other color other than white
    // And block itself contains buffer number and how many times the buffer is called

    use ratatui::widgets::Block;

    //TODO: is there advantage in spliting ui and data
    struct ShuffleList<'a> {
        // obtained from db
        raw_buffer: Vec<String>,
        // The actual block inside which every thing will be drawn
        // FIXME: This block needs to be split into more blocks
        main_block: Block<'a>,
        // order of text this will work with selected to determine how many to color
        order_of_blobs: Vec<usize>,
        // total number of selected blobs
        selected: usize,
    }

    impl<'a> ShuffleList<'a> {
        fn clip_text_to_draw() {
            todo!("Clip text to draw in lazy manner, with some kind of cache.")
        }

        fn change_style_on_select() {
            todo!("Change borders style on selection")
        }

        fn shuffle_blobs() {
            todo!("Change order of shuffled blobs and handle the value of selected")
        }
    }
}
