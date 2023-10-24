extern crate notify;
extern crate chrono;

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::fs;
use std::path::Path;
use chrono::{Utc, Datelike, Timelike};

fn main() {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, std::time::Duration::new(1, 0)).unwrap();

    watcher.watch("/path/to/directory", RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                handle_event(event);
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn handle_event(event: notify::DebouncedEvent) {
    match event {
        notify::DebouncedEvent::Create(path) => {
            sort_file(path);
        },
        _ => (),
    }
}

fn sort_file(path: std::path::PathBuf) {
    let metadata = fs::metadata(&path).unwrap();
    let created = metadata.created().unwrap();
    let datetime = chrono::DateTime::<Utc>::from(created);
    
    let dest_dir = format!(
        "/sorted_files/{}/{}/{}/{}",
        datetime.year(),
        datetime.month(),
        datetime.day(),
        datetime.hour()
    );
    
    fs::create_dir_all(&dest_dir).unwrap();
    let dest_path = Path::new(&dest_dir).join(path.file_name().unwrap());
    fs::rename(path, dest_path).unwrap();
}
