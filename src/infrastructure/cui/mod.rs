use crate::domain::models::youtube;
use std::io::stdin;

#[allow(dead_code)]
pub fn receive_youtube_id_from_user_by_stdin() -> youtube::video::id::Id {
    let mut youtube_id = String::new();
    println!("please enter an id of youtube video: ");
    stdin()
        .read_line(&mut youtube_id)
        .expect("Failed to read line");
    println!("input: \"{}\"", youtube_id);
    youtube::video::id::new(&youtube_id.trim())
}

pub fn receive_youtube_id_from_user_by_args() -> youtube::video::id::Id {
    let args: Vec<String> = std::env::args().collect();
    let youtube_id = &args[1];
    println!("youtube_id: \"{}\"", youtube_id);
    youtube::video::id::new(&youtube_id.trim())
}
