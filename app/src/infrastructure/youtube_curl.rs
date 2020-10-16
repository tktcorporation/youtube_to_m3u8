use crate::domain;
use domain::types::m3u8_url;
use domain::types::youtube;

pub async fn get_m3u8(id: &youtube::video::id::Id) -> m3u8_url::M3U8url {
    m3u8_url::parse(&request(id).await)
}

async fn request(id: &youtube::video::id::Id) -> String {
    let mut url = String::from("https://www.youtube.com/get_video_info?video_id=");
    url.push_str(&id.to_string());
    let client = reqwest::Client::new();
    let res = client.get(&url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Safari/605.1.15").send().await.unwrap();
    res.text().await.unwrap()
}

#[cfg(test)]
mod tests {
    use crate::domain::types::youtube::video::id;
    use crate::infrastructure::youtube_curl;

    #[tokio::test]
    async fn it_request() {
        let id = id::new("rvkxtVkvawc");
        let url = youtube_curl::request(&id);
        assert_eq!(true, url.await.len() > 100);
    }

    #[tokio::test]
    async fn it_get_m3u8() {
        let id = id::new("rvkxtVkvawc");
        let m3u8url = youtube_curl::get_m3u8(&id);
        assert_eq!(true, m3u8url.await.get_url().len() > 20);
    }
}
