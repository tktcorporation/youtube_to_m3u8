// use curl::easy::Easy;
// use std::io::{stdin, stdout, Write};
use infrastructure::youtube_curl::get_video_info;

mod domain;
mod infrastructure;

fn main() {
    // let mut youtube_id = String::new();
    // stdin()
    //     .read_line(&mut youtube_id)
    //     .expect("Failed to read line");

    let info = get_video_info();
    println!("{}", info);
}
