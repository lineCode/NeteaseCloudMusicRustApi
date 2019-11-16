use actix_web::{
    App, HttpRequest, FromRequest, HttpResponse,
    HttpServer, Responder, web, error,
    cookie,
};
use actix_web::http::{ Uri, };
use reqwest::header::{HOST, CONTENT_TYPE, USER_AGENT, REFERER, HeaderMap, ToStrError, CONTENT_ENCODING};
use reqwest::blocking::{
    ClientBuilder,
    Client,
};
use serde::Deserialize;

use super::crypto::Crypto;
use actix_web::error::UrlencodedError::ContentType;
use crate::crypto::HashType;
use base64::CharacterSet::Crypt;
use rand::rngs::OsRng;
use rand::Rng;

const linux_user_agent: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.90 Safari/537.36";

const user_agent_list: [&str; 14] = [
    "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0 Mobile/13B143 Safari/601.1",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0 Mobile/13B143 Safari/601.1",
    "Mozilla/5.0 (Linux; Android 5.0; SM-G900P Build/LRX21T) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 5.1.1; Nexus 6 Build/LYZ28E) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Mobile Safari/537.36",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 10_3_2 like Mac OS X) AppleWebKit/603.2.4 (KHTML, like Gecko) Mobile/14F89;GameHelper",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 10_0 like Mac OS X) AppleWebKit/602.1.38 (KHTML, like Gecko) Version/10.0 Mobile/14A300 Safari/602.1",
    "Mozilla/5.0 (iPad; CPU OS 10_0 like Mac OS X) AppleWebKit/602.1.38 (KHTML, like Gecko) Version/10.0 Mobile/14A300 Safari/602.1",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.12; rv:46.0) Gecko/20100101 Firefox/46.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.115 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_5) AppleWebKit/603.2.4 (KHTML, like Gecko) Version/10.1.1 Safari/603.2.4",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:46.0) Gecko/20100101 Firefox/46.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/13.1058",
];

pub const topList: [&str; 24] = [
    "3779629", //云音乐新歌榜
    "3778678", //云音乐热歌榜
    "2884035", //云音乐原创榜
    "19723756", //云音乐飙升榜
    "10520166", //云音乐电音榜
    "180106", //UK排行榜周榜
    "60198", //美国Billboard周榜
    "21845217", //KTV嗨榜
    "11641012", //iTunes榜
    "120001", //Hit FM Top榜
    "60131", //日本Oricon周榜
    "3733003", //韩国Melon排行榜周榜
    "60255", //韩国Mnet排行榜周榜
    "46772709", //韩国Melon原声周榜
    "112504", //中国TOP排行榜(港台榜)
    "64016", //中国TOP排行榜(内地榜)
    "10169002", //香港电台中文歌曲龙虎榜
    "4395559", //华语金曲榜
    "1899724", //中国嘻哈榜
    "27135204", //法国 NRJ EuroHot 30周榜
    "112463", //台湾Hito排行榜
    "3812895", //Beatport全球电子舞曲榜
    "71385702", //云音乐ACG音乐榜
    "991319590" //云音乐嘻哈榜
];


pub fn create_request<T: ToString>(
    method: &str,
    ua: &str,
    crypto: &str,
    url: &str,
    value: &T
    ) -> serde_json::Value {

    let mut headers = HeaderMap::new();

    if method.to_uppercase() ==  "POST" {
        headers.insert(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded".parse().unwrap());
    }
    if url.contains("music.163.com") {
        headers.insert(
            REFERER,
            "https://music.163.com".parse().unwrap());
    }
    headers.insert(
        CONTENT_ENCODING,
        "utf-8".parse().unwrap()
    );

    let _ = match crypto {
        "linuxapi" => headers.insert(
                USER_AGENT,
                linux_user_agent.parse().unwrap() ),
        _ => headers.insert(
            USER_AGENT,
            choose_user_agent(ua).parse().unwrap() )
    };


    let body = match crypto {
        "weapi" => Crypto::weapi(&value.to_string()),
        "linuxapi" => {
            let data = format!(
                r#"{{"method":"{}","url":"{}","params":{}}}"#,
                method,
                url.replace("weapi", "api"),
                value.to_string() );
            println!("data={}", data);
            Crypto::linuxapi(&data)
        },
        _ => Crypto::weapi(&value.to_string()),
    };

    let url = match crypto {
        "linuxapi" => {
           "https://music.163.com/api/linux/forward"
        },
        _ => url,
    };

    println!("url:{}; body={}",url, body);

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    client.post(url)
        .body(body)
        .send().unwrap()
        .json().unwrap()
}

fn choose_user_agent(ua: &str) -> &str {
    let index = if ua == "mobile" {
        rand::thread_rng().gen_range(0,7)
    } else if ua == "pc" {
        rand::thread_rng().gen_range(0, 5) + 8
    } else {
        rand::thread_rng().gen_range(0, user_agent_list.len())
    };

    println!("userAgent={}", user_agent_list[index]);

    unsafe {
        user_agent_list.get_unchecked(index)
    }
}

#[derive(Deserialize)]
pub struct Identity {
    pub id: i32,
    pub other: Option<i32>
}

impl ToString for Identity {
    fn to_string(&self) -> String {
        format!(r#"{{"id":"{}"}}"#, self.id)
    }
}

#[derive(Deserialize)]
pub struct PageIndex {
    page: i32
}

impl ToString for PageIndex {
    fn to_string(&self) -> String {
        format!(r#"{{"page":"{}"}}"#, self.page)
    }
}

#[derive(Deserialize)]
pub struct CateId {
    cateId: i32
}

impl ToString for CateId {
    fn to_string(&self) -> String {
        format!(r#"{{"cateId":"{}"}}"#, self.cateId)
    }
}


#[derive(Deserialize)]
pub struct TopList {
    id: String,
    n: Option<i32>
}

impl ToString for TopList {
    fn to_string(&self) -> String {
        format!(r#"{{"id":"{}","n":"{}"}}"#, self.id, self.n.unwrap_or(10000))
    }
}

#[derive(Deserialize)]
pub struct NewSong {
    id: String,
    total: Option<bool>
}

impl ToString for NewSong {
    fn to_string(&self) -> String {
        format!(r#"{{"areadId":"{}","total":"{}"}}"#, self.id, self.total.unwrap_or(true))
    }
}

#[derive(Deserialize)]
pub struct SongInfo {
    id: String,
    br: Option<u32>,
}

impl ToString for SongInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"ids":"[{}]","br":{}}}"#, self.id, self.br.unwrap_or(999000))
    }
}

#[derive(Deserialize)]
pub struct DjInfo {
    pub limit: Option<i32>,
    pub offset: Option<i32>
}

impl ToString for DjInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"limit":"{}","offset":"{}"}}"#,
            self.limit.unwrap_or(30),
            self.offset.unwrap_or(0)
        )
    }
}

#[derive(Deserialize)]
pub struct MvInfo {
    id: String,
    r: Option<u32>,
}

impl ToString for MvInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"id":"{}","r":{}}}"#, self.id, self.r.unwrap_or(1080))
    }
}

#[derive(Deserialize)]
pub struct TopMvInfo {
    limit: Option<u32>,
    offset: Option<u32>,
    total: Option<bool>,
}

impl ToString for TopMvInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"limit":"{}","offset":"{}","total":"{}"}}"#,
            self.limit.unwrap_or(30),
            self.offset.unwrap_or(0),
            self.total.unwrap_or(true)
        )
    }
}

#[derive(Deserialize)]
pub struct ArtistAlbum {
    pub id: u32,
    limit: Option<u32>,
    offset: Option<u32>,
    total: Option<bool>,
}

impl ToString for ArtistAlbum {
    fn to_string(&self) -> String {
        format!(r#"{{"limit":"{}","offset":"{}","total":"{}"}}"#,
                self.limit.unwrap_or(50),
                self.offset.unwrap_or(0),
                self.total.unwrap_or(true)
        )
    }
}

#[derive(Deserialize)]
pub struct TopAlbumInfo {
    area: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    total: Option<bool>,
}

impl ToString for TopAlbumInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"area":"{}","limit":"{}","offset":"{}","total":"{}"}}"#,
            self.area.as_ref().unwrap_or(&String::from("ALL")),
            self.limit.unwrap_or(50),
            self.offset.unwrap_or(0),
            self.total.unwrap_or(true)
        )
    }
}

#[derive(Deserialize)]
pub struct CommentInfo {
    pub id: String,
    limit: Option<u32>,
    offset: Option<u32>,
    before: Option<u32>,
}

impl ToString for CommentInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"rid":"{}","limit":"{}","offset":"{}"}}"#,
            self.id,
            self.limit.unwrap_or(20),
            self.offset.unwrap_or(0)
        )
    }
}

#[derive(Deserialize)]
pub struct EmailLoginInfo {
    email: String,
    password: String,
    rememberLogin: Option<bool>,
}

impl ToString for EmailLoginInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"username":"{}","password":"{}","rememberLogin":"{}"}}"#,
                self.email,
                Crypto::hash_encrypt(&self.password, HashType::md5, hex::encode),
                self.rememberLogin.unwrap_or(true)
        )
    }
}

#[derive(Deserialize)]
pub struct CellPhoneLoginInfo {
    phone: String,
    password: String,
    countryCode: Option<u16>,
    rememberLogin: Option<bool>,
}

impl ToString for CellPhoneLoginInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"phone":"{}","password":"{}","rememberLogin":"{}"}}"#,
            self.phone,
            Crypto::hash_encrypt(&self.password, HashType::md5, hex::encode),
            self.rememberLogin.unwrap_or(true)
        )
    }
}

#[derive(Deserialize)]
pub struct SearchInfo {
    keywords: String,
    types: Option<u32>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl ToString for SearchInfo {
    fn to_string(&self) -> String {
        format!(r#"{{"s":"{}","type":"{}","limit":"{}","offset":"{}"}}"#,
            self.keywords,
            self.types.unwrap_or(1),
            self.limit.unwrap_or(30),
            self.offset.unwrap_or(0)
        )
    }
}

#[derive(Deserialize)]
pub struct Identify {
    id: u32
}

impl ToString for Identify {
    fn to_string(&self) -> String {
        format!(r#"{{"id":"{}"}}"#, self.id)
    }
}