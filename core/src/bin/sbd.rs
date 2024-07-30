use config::Base;
use core::db::Db;
use hooks::Clipboard;

fn main() {
    // let logfile = fs::File::open("/tmp/sblog");
    // create a new process
    let mut clipboard = Clipboard::new(hooks::Environment::Wayland, 2);
    let pastebin = Db::new_connection(Base::default()).unwrap();
    loop {
        if let Ok(paste) = clipboard.poll() {
            if let Ok(to_string) = String::from_utf8(paste) {
                if let Err(_) = pastebin.push(to_string) {
                    dbg!("Push to db failed");
                }
            } else {
                dbg!("String parsing failed");
            }
        } else {
            dbg!("Polling failed");
        }
    }
}
