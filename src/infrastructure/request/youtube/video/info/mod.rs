use crate::domain;
use crate::infrastructure::request::youtube::BASE_URL;
use domain::models::youtube;

const MAC_SAFARI_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Safari/605.1.15";
const PATH_OF_GET_VIDEO_INFO: &str = "/get_video_info";
const KEY_OF_VIDEO_ID: &str = "video_id";

pub async fn request(id: &youtube::video::id::Id) -> Result<youtube::video::info::Info, String> {
    let mut url = String::from(BASE_URL);
    url.push_str(PATH_OF_GET_VIDEO_INFO);
    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, MAC_SAFARI_USER_AGENT)
        .query(&[(KEY_OF_VIDEO_ID, id.to_string())])
        .send()
        .await
        .unwrap();
    match res.status() {
        reqwest::StatusCode::OK => {},
        _ => return Err(format!("status code: {}", res.status())),
    };
    let body = &res.text().await.unwrap();

    match youtube::video::info::new(body) {
        Ok(info) => Ok(info),
        Err(e) => Err(e.to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::youtube::video::info;

    #[test]
    fn it_string_join() {
        let mut url = String::from(BASE_URL);
        url.push_str(PATH_OF_GET_VIDEO_INFO);
        assert_eq!("https://www.youtube.com/get_video_info", url);
    }

    #[tokio::test]
    async fn it_request() {
        let info = request(&youtube::video::id::new("rvkxtVkvawc"))
            .await.unwrap();
        assert_eq!(info, info::new("rvkxtVkvawc").unwrap());
    }

    #[tokio::test]
    async fn it_request_fail() {
        let is = request(&youtube::video::id::new("xxxx")).await.is_err();
        assert_eq!(is, true);
    }
    // use super::*;
    // use crate::domain::models::youtube::video::id;
}
