use infrastructure::cui;
use infrastructure::youtube_curl;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    let youtube_id = cui::receive_youtube_id_from_user();
    let info = youtube_curl::get_m3u8(&youtube_id);
    println!("{}", info.await.get_url());
}
