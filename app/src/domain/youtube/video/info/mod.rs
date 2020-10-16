use crate::domain::types::url::m3u8;
use regex::Regex;

const NOT_FOUND_M3U8: &str = "A m3u8 url is not found: {}";

pub struct Info {
    body: String,
}
pub fn new(body: &str) -> Result<Info, &str> {
    let re = Regex::new(r"(?P<status>status=(ok|fail))").unwrap();
    let caps = re.captures(body).unwrap();
    if caps["status"].to_string() == "status=fail" {
        return Err("Invalid parameters body was received.");
    }
    Ok(Info {
        body: body.to_string(),
    })
}
impl Info {
    pub fn extract_m3u8_url(&self) -> m3u8::M3U8 {
        let re = Regex::new(r"(?P<url>https%3A%2F%2Fmanifest.googlevideo.com.+m3u8)").unwrap();
        println!("{}", &self.body);
        let caps = re.captures(&self.body).expect(NOT_FOUND_M3U8);
        m3u8::new(&caps["url"].to_string())
    }
    pub fn get_body(&self) -> &str {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_new_success() {
        let info = new("status=ok").unwrap();
        assert_eq!("status=ok", info.get_body());
    }

    #[test]
    fn it_new_fail() {
        let is = new("status=fail").is_err();
        assert_eq!(is, true);
    }

    #[test]
    fn it_extract_m3u8_url() {
        let info =
            new("status=ok&xxxxxhttps%3A%2F%2Fmanifest.googlevideo.com/xxxxxxxxxxxxxxxxxx.m3u8xxx")
                .unwrap();
        assert_eq!(
            "https%3A%2F%2Fmanifest.googlevideo.com/xxxxxxxxxxxxxxxxxx.m3u8",
            info.extract_m3u8_url().to_string()
        );
    }
}
