use curl::easy::Easy;
use std::io::{stdout, Write};

pub fn get_video_info() -> u32 {
    let mut easy = Easy::new();
    easy.url("https://www.rust-lang.org/").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
    match easy.response_code() {
        Ok(info) => info,
        Err(error) => panic!("It fail to fetch a movie info: {}", error),
    }
}

pub fn curl_sample() -> u32 {
    let mut easy = Easy::new();
    easy.url("https://www.rust-lang.org/").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
    match easy.response_code() {
        Ok(info) => info,
        Err(error) => panic!("It fail to fetch a movie info: {}", error),
    }
}
