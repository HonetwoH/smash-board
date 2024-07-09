use smash_board::hook::poll_clipboard;

fn main() {
    loop {
        _ = poll_clipboard();
    }
}
