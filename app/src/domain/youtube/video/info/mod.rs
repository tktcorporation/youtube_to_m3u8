use crate::domain::types::url::m3u8;
use regex::Regex;

pub struct Info {
    body: String,
}
pub fn new(body: &str) -> Info {
    Info {
        body: body.to_string(),
    }
}
impl Info {
    pub fn extract_m3u8_url(&self) -> m3u8::M3U8 {
        let re = Regex::new(r"(?P<url>https%3A%2F%2Fmanifest.googlevideo.com.+m3u8)").unwrap();
        let caps = re.captures(&self.body).unwrap();
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
    fn it_extract_m3u8_url() {
        let info = new("xxxxxhttps%3A%2F%2Fmanifest.googlevideo.com/xxxxxxxxxxxxxxxxxx.m3u8xxx");
        assert_eq!(
            "https%3A%2F%2Fmanifest.googlevideo.com/xxxxxxxxxxxxxxxxxx.m3u8",
            info.extract_m3u8_url().to_string()
        );
    }
}
