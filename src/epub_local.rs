use epub::doc::EpubDoc;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

pub fn read_and_write(path: PathBuf) {
    let doc = EpubDoc::new(&path);
    let mut doc = doc.unwrap();

    let mut new_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/foo.html")
        .unwrap();

    for _ in 0..doc.get_num_pages() {
        if let Err(e) = writeln!(new_file, "{:#?}",&doc.get_current_str().unwrap().0) {
            eprintln!("Couldn't write to file: {}", e);
        };

        doc.go_next();
        println!("\n\naaaaaaaaaaaaaaa\n\n");
    }
}



