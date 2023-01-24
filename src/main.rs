use std::path::PathBuf;
use std::env;
use tts::*;

mod epub_local;
fn main() {
    let path = PathBuf::from(env::args_os().nth(1).expect("no file given"));

    epub_local::read_and_write(path);

    let mut tts = Tts::default();
    tts.expect("").speak("hello world".to_string(), false);
}
