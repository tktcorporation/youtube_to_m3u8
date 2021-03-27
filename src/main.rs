use infrastructure::cui;
use infrastructure::request;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() {
    let youtube_id = cui::receive_youtube_id_from_user_by_args();
    let info = request::youtube::video::info::request(&youtube_id).await;
    let result = match info {
        Ok(info) => Ok(info.extract_m3u8_url().url_decode()),
        Err(err) => Err(err),
    };
    match result {
        Ok(result) => println!("result: {}", result),
        Err(err) => println!("error: {}", err),
    }
}
