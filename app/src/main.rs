use infrastructure::youtube_curl;

mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    // let mut youtube_id = String::new();
    // stdin()
    //     .read_line(&mut youtube_id)
    //     .expect("Failed to read line");

    let info = youtube_curl::get_m3u8();
    println!("{}", info.await.get_url());
}
