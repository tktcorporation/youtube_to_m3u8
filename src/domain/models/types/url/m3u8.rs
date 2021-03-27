use percent_encoding::percent_decode_str;
use regex::Regex;

const NOT_FOUND_M3U8: &str = "invalid m3u8 url is received: {}";

#[derive(Debug,PartialEq)]
pub struct M3U8 {
    value: String,
}

pub fn new(st: &str) -> M3U8 {
    let re = Regex::new(r"(?P<url>https%3A%2F%2F.+m3u8)").unwrap();
    re.captures(st).expect(NOT_FOUND_M3U8);
    M3U8 {
        value: st.to_string(),
    }
}

impl M3U8 {
    pub fn url_decode(&self) -> String {
        percent_decode_str(&self.value)
            .decode_utf8()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_url_decode() {
        let m3u8 =
            new("https%3A%2F%2Fmanifest.googlevideo.com%2Fapi%2Fmanifest%2Fhls_variant%2Fexpire%2F1602895282%2Fei%2FUumJX_K9CMWClQTK_KXwCw%2Fip%2F126.255.82.97%2Fid%2Faef931b5592f6b07%2Fsource%2Fyoutube%2Frequiressl%2Fyes%2Fplayback_host%2Fr3---sn-ogul7n7s.googlevideo.com%2Fmh%2F12%2Fmm%2F31%252C26%2Fmn%2Fsn-ogul7n7s%252Csn-3pm7kn7l%2Fms%2Fau%252Conr%2Fmv%2Fm%2Fmvi%2F3%2Fpl%2F16%2Fhfr%2F1%2Ftts_caps%2F1%2Fmaudio%2F1%2Fpcm2%2Fno%2Finitcwndbps%2F1066250%2Fvprv%2F1%2Fgo%2F1%2Fmt%2F1602873530%2Ffvip%2F3%2Fkeepalive%2Fyes%2Ffexp%2F23915654%2Fdover%2F11%2Fitag%2F0%2Fplaylist_type%2FDVR%2Fsparams%2Fexpire%252Cei%252Cip%252Cid%252Csource%252Crequiressl%252Chfr%252Ctts_caps%252Cmaudio%252Cpcm2%252Cvprv%252Cgo%252Citag%252Cplaylist_type%2Fsig%2FAOq0QJ8wRgIhALP940xJgYfLwI5OEiwSxeAKfIbKmqvjr4-xOY98xsiZAiEAve9oHPNC5TI2ML6JiDDARmQw06JwLocYAGrew9mZ9WY%253D%2Flsparams%2Fplayback_host%252Cmh%252Cmm%252Cmn%252Cms%252Cmv%252Cmvi%252Cpl%252Cinitcwndbps%2Flsig%2FAG3C_xAwRQIgX6QfinuK6KPJhKIB_sPU6aAlgaN7Q_BXae9zb797bXgCIQDqgVejz_svO8q6FWZVwKDCreMQYQC_LN413AdwN1zvrg%253D%253D%2Ffile%2Findex.m3u8");
        assert_eq!(
            "https://manifest.googlevideo.com/api/manifest/hls_variant/expire/1602895282/ei/UumJX_K9CMWClQTK_KXwCw/ip/126.255.82.97/id/aef931b5592f6b07/source/youtube/requiressl/yes/playback_host/r3---sn-ogul7n7s.googlevideo.com/mh/12/mm/31%2C26/mn/sn-ogul7n7s%2Csn-3pm7kn7l/ms/au%2Conr/mv/m/mvi/3/pl/16/hfr/1/tts_caps/1/maudio/1/pcm2/no/initcwndbps/1066250/vprv/1/go/1/mt/1602873530/fvip/3/keepalive/yes/fexp/23915654/dover/11/itag/0/playlist_type/DVR/sparams/expire%2Cei%2Cip%2Cid%2Csource%2Crequiressl%2Chfr%2Ctts_caps%2Cmaudio%2Cpcm2%2Cvprv%2Cgo%2Citag%2Cplaylist_type/sig/AOq0QJ8wRgIhALP940xJgYfLwI5OEiwSxeAKfIbKmqvjr4-xOY98xsiZAiEAve9oHPNC5TI2ML6JiDDARmQw06JwLocYAGrew9mZ9WY%3D/lsparams/playback_host%2Cmh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps/lsig/AG3C_xAwRQIgX6QfinuK6KPJhKIB_sPU6aAlgaN7Q_BXae9zb797bXgCIQDqgVejz_svO8q6FWZVwKDCreMQYQC_LN413AdwN1zvrg%3D%3D/file/index.m3u8",
            m3u8.url_decode()
        );
    }
}
