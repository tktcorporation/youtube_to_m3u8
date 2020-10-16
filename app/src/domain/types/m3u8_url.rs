use regex::Regex;

pub struct M3U8url {
    value: String,
}

pub fn parse(string: &str) -> M3U8url {
    let re = Regex::new(r"(?P<url>https%3A%2F%2Fmanifest.googlevideo.com.+m3u8)").unwrap();
    let caps = re.captures(&string).unwrap();
    M3U8url {
        value: caps["url"].to_string(),
    }
}

impl M3U8url {
    pub fn get_url(&self) -> String {
        self.value.clone()
    }
}

// impl M3U8url {}

#[cfg(test)]
mod tests {
    use crate::domain::types::m3u8_url::parse;
    #[test]
    fn it_parse() {
        let url = parse("xxxxxhttps%3A%2F%2Fmanifest.googlevideo.com/xxxxxxxxxxxxxxxxxx.m3u8xxx");
        assert_eq!(
            "https%3A%2F%2Fmanifest.googlevideo.com/xxxxxxxxxxxxxxxxxx.m3u8",
            url.value
        );
    }
}
