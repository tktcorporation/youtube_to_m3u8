use infrastructure::cui;
use infrastructure::request;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    let youtube_id = cui::receive_youtube_id_from_user();
    let info = request::youtube::video::info::request(&youtube_id);
    println!("{}", info.await.extract_m3u8_url().url_decode());
}
