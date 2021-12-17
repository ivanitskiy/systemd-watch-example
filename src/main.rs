use std::process;

use systemd::{Journal, journal};



fn main() {
    println!("Hello, world!");

    // let mut journal = Journal::open(JournalFiles::All, false, true)
    //     .expect("Failed to open systemd journal");

    let mut j = journal::OpenOptions::default().local_only(true).current_user(true).open().unwrap();

    match j.seek_head() {
        Ok(cursor) => println!("opened {}", cursor),
        Err(e) => {
            print!("failed {}", e);
            process::exit(1)
        }
    }
}
