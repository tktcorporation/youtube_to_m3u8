use crate::domain::youtube;
use std::io::stdin;

pub fn receive_youtube_id_from_user() -> youtube::video::id::Id {
    let mut youtube_id = String::new();
    println!("please enter an id of youtube video: ");
    stdin()
        .read_line(&mut youtube_id)
        .expect("Failed to read line");
    println!("input: \"{}\"", youtube_id);
    youtube::video::id::new(&youtube_id.trim())
}
