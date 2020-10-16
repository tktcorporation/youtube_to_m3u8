use crate::domain;
use domain::types::m3u8_url;

pub async fn get_m3u8() -> m3u8_url::M3U8url {
    m3u8_url::parse(&request("rvkxtVkvawc").await)
}

async fn request(id: &str) -> String {
    // let mut url = String::from("https://www.youtube.com/get_video_info?video_id=");
    // url.push_str(id);
    let client = reqwest::Client::new();
    let res = client.get("https://www.youtube.com/get_video_info?video_id=rvkxtVkvawc")
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Safari/605.1.15").send().await.unwrap();
    res.text().await.unwrap()
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::youtube_curl;

    #[tokio::test]
    async fn it_request() {
        let url = youtube_curl::request("rvkxtVkvawc");
        assert_eq!(
            "https%3A%2F%2Fmanifest.googlevideo.com/xxxxxxxxxxxxxxxxxx.m3u8",
            url.await
        );
    }

    #[tokio::test]
    async fn it_get_m3u8() {
        let url = youtube_curl::get_m3u8();
        assert_eq!(
            "https%3A%2F%2Fmanifest.googlevideo.com/xxxxxxxxxxxxxxxxxx.m3u8",
            url.await.get_url()
        );
    }
}
