use std::error::Error;
use systemd::{journal, JournalRecord};

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = journal::OpenOptions::default()
        .local_only(true)
        .current_user(true)
        .open()
        .unwrap();

    reader.seek_tail().expect("Could not open systemd");

    reader
        .watch_all_elements(|record: JournalRecord| {
            let msg = record.get("MESSAGE").unwrap();
            println!("got message:{}", msg);
            Ok(())
        })
        .unwrap_or_else(|e| {
            println!("Stopped watching systemd: {}", e);
        });

    println!("End of example.");
    Ok(())
}
