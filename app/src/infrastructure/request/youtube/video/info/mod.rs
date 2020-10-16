use crate::domain;
use crate::infrastructure::request::youtube::BASE_URL;
use domain::youtube;

const MAC_SAFARI_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Safari/605.1.15";
const PATH_OF_GET_VIDEO_INFO: &str = "/get_video_info";
const KEY_OF_VIDEO_ID: &str = "video_id";

pub async fn request(id: &youtube::video::id::Id) -> youtube::video::info::Info {
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
    youtube::video::info::new(&res.text().await.unwrap())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::domain::youtube::video::id;

//     #[tokio::test]
//     async fn it_request() {
//         let id = id::new("rvkxtVkvawc");
//         let info = request(&id);
//         assert_eq!(true, info.await.get_body().len() > 100);
//     }
// }
