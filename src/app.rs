
use actix_web::{
    HttpRequest, HttpResponse, HttpServer,
    Responder, App, FromRequest,
    web, error, middleware, get,
};
use actix_web::http::{ Uri, };
use reqwest::header::{
    HOST, CONTENT_TYPE, USER_AGENT, REFERER,
    HeaderMap
};
use serde::Deserialize;
use listenfd::ListenFd;
use urlqstring::QueryParams;
use crate::{
    api,
    api::{
        banner_type, operator, resource_type, topList,
    }
};
use crate::crypto::{
    Crypto, HashType
};
use actix_web::web::{service, to};
use actix_http::error::PayloadError::Http2Payload;
use actix_http::http::HeaderValue;
use lazy_static::lazy_static;
use actix_http::cookie::Cookie;
use actix_web::dev::RequestHead;
use std::ops::Deref;
use actix_web::error::UrlencodedError::ContentType;
use base64::CharacterSet::Crypt;

const CONTENT_TP: &'static str = "application/json; charset=utf-8";

fn index_root() -> impl Responder {
    println!("index_root.....");
    HttpResponse::Ok().body("Hello World!")
}

#[get("/activate/init/profile")]
fn index_activate_init_profile( req: HttpRequest ) -> impl Responder {
    let url = "http://music.163.com/eapi/activate/initProfile";
    let value = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "eapi",
                url,
                &value
            )
        )
}

#[get("/album/detail/dynamic")]
fn index_album_detail_dynamic(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/album/detail/dynamic";
    let value = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &value
            )
        )
}

#[get("/album/newest")]
fn index_album_newest(msg: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/discovery/newAlbum";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/album/sub")]
fn index_album_sub(req: HttpRequest ) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let url = format!("https://music.163.com/api/album/{}", id);
    let value = QueryParams::from(query).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &value
            )
        )
}

#[get("/album/sublist")]
fn index_album_sublist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/album/sublist";
    let query = QueryParams::from(req.query_string());

    let limit = query.value("limit").unwrap_or("25");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;

    let value = QueryParams::from(
        &format!("limit={}&offset={}&total={}", limit, offset, total)
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &value
            )
        )
}

#[get("/album")]
fn index_album(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap_or("32311");
    let url = &format!("https://music.163.com/weapi/v1/album/{}", id);
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/artist/album")]
fn index_artist_album(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap_or("6452");
    let url = format!("https://music.163.com/weapi/artist/albums/{}",id);

    let limit = query.value("limit").unwrap_or("30");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;

    let value = format!("limit={}&offset={}&total={}", limit,offset,total);
    let info = QueryParams::from(&value).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/artist/desc")]
fn index_artist_desc(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/introduction";
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap_or("6452");
    let info = QueryParams::from(&format!("id={}", id)).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/artist/list")]
fn index_artist_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/list";
    let query = QueryParams::from(req.query_string());

    let categoryCode = query.value("cat").unwrap_or("1001");
    let offset = query.value("offset").unwrap_or("0");
    let limit = query.value("limit").unwrap_or("30");
    let total = true;
    let initial = query.value("initial").unwrap_or("undefined");

    let info = QueryParams::from(
        &format!("categoryCode={}&initial={}&offset={}&limit={}&total={}",
            categoryCode, initial, offset, limit, total
        )
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/artist/mv")]
fn index_artist_mv(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/mvs";
    let query =  QueryParams::from(req.query_string());

    let artistId = query.value("id").unwrap_or("6452");
    let limit = query.value("limit").unwrap_or("30");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;

    let info = QueryParams::from(
        &format!("artistId={}&limit={}&offset={}&total={}",
            artistId, limit, offset, total
        )
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/artist/sub")]
fn index_artist_sub(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let artistId = query.value("id").unwrap_or("6452");
    let t = query.value("t").unwrap_or("0");
    let sub = if t.parse::<i32>().unwrap() == 1 {
        "sub"
    } else {
        "unsub"
    };
    let url = format!("https://music.163.com/weapi/artist/{}", sub);

    let info = QueryParams::from(
        &format!("artistId={}&artistIds=[{}]", artistId, artistId)
    ).json();
    println!("info:{}", info);
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/artist/sublist")]
fn index_artist_sublist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/sublist";
    let query = QueryParams::from(req.query_string());

    let limit = query.value("limit").unwrap_or("25");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;

    let info = QueryParams::from(
        &format!("limit={}&offset={}&total={}", limit, offset, total)
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/artist/top/song")]
fn index_artist_top_song(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/artist/top/song";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/artists")]
fn index_artists(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap_or("6452");
    let url = format!("https://music.163.com/weapi/v1/artist/{}", id);
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &"{}"
            )
        )
}

#[get("/banner")]
fn index_banner(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/v2/banner/get";
    let id = QueryParams::from(req.query_string()).value("type").unwrap_or("0")
        .parse::<usize>().unwrap_or(0);
    let _type = banner_type[ if id > 3 { 0 } else { id } ];
    let info = QueryParams::from(
        &format!("clientType={}",_type)
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "linuxapi",
                url,
                &info
            )
        )
}

#[get("/batch")]
fn index_batch(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/eapi/batch";

}
#[get("/captcha/register")]
fn index_captcha_register(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/register/cellphone";
    let query = QueryParams::from(req.query_string());

    let captcha = query.value("captcha").unwrap();
    let phone = query.value("phone").unwrap();
    let password = query.value("password").unwrap();
    let pw = Crypto::hash_encrypt(password, HashType::md5, hex::encode);
    let nickname = query.value("nickname").unwrap();

    let info = QueryParams::from(
        &format!("captcha={}&phone={}&password={}&nickname={}",
            captcha, phone, pw, nickname
        )
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/captcha/sent")]
fn index_captcha_sent(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/sms/captcha/sent";
    let query = QueryParams::from(req.query_string());

    let ctcode = query.value("ctcode").unwrap_or("86");
    let cellphone = query.value("phone").unwrap();

    let info = QueryParams::from(
        &format!("ctcode={}&cellphone={}", ctcode, cellphone)
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/captcha/verify")]
fn index_captcha_verify(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/sms/captcha/verify";
    let query = QueryParams::from(req.query_string());

    let ctcode = query.value("ctcode").unwrap_or("86");
    let cellphone = query.value("phone").unwrap();
    let captcha = query.value("captcha").unwrap();

    let info = QueryParams::from(
        &format!("ctcode={}&cellphone={}&captcha={}",
            ctcode, cellphone, captcha
        )
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/cellphone/existence/check")]
fn index_cellphone_existence_check(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/eapi/cellphone/existence/check";
    let query = QueryParams::from(req.query_string());

    let countrycode = query.value("countrycode").unwrap_or("86");
    let cellphone = query.value("phone").unwrap();

    let info = QueryParams::from(
        &format!("countrycode={}&cellphone={}", countrycode, cellphone)
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "eapi",
                url,
                &info
            )
        )
}

#[get("/check/music")]
fn index_check_music(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/song/enhance/player/url";
    let query = QueryParams::from(req.query_string());

    let ids = query.value("id").unwrap();
    let br = query.value("br").unwrap_or("999000");

    let info = QueryParams::from(
        &format!("ids={}&br={}", ids, br)
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/comment")]
fn index_comment(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = operator[ query.value("t").unwrap().parse::<usize>().unwrap_or(0) ];
    let url = format!("https://music.163.com/weapi/resource/comments/{}", t);

    let rtype = resource_type[ query.value("type").unwrap().parse::<usize>().unwrap_or(0) ];
    let threadId:String = if rtype == "A_EV_2_" {
        String::from(query.value("threadId").unwrap())
    } else {
        rtype.to_owned() + query.value("id").unwrap()
    };
    let mut res = threadId;
    if t == "add" {
        res.push_str(
            &format!("&content={}",
                     query.value("content").unwrap()
            )
        );
    } else if t == "delete" {
        res.push_str(
            &format!("&commentId={}",
                query.value("commentId").unwrap()
            )
        );
    } else if t == "reply" {
        res.push_str(
            &format!("&commentId={}&content={}",
                query.value("commentId").unwrap(),
                query.value("content").unwrap()
            )
        );
    }

    let info = QueryParams::from(&res).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/comment/album")]
fn index_comment_album(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_AL_3_{}", id);

    let info = QueryParams::from(
        &format!("rid={}", id)
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/comment/dj")]
fn index_comment_dj(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/A_DJ_1_{}", id);
    let info = QueryParams::from(query).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/comment/event")]
fn index_comment_event(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("threadId").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/{}", id);

    let info = QueryParams::from(query).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/comment/hot")]
fn index_comment_hot(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let _type = resource_type[query.value("type").unwrap().parse::<usize>().unwrap_or(0)];
    let id = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/hotcomments/{}{}",
        _type, id
    );

    let limit = query.value("limit").unwrap_or("20");
    let offset = query.value("offset").unwrap_or("0");
    let beforeTime = query.value("beforeTime").unwrap_or("0");

    let info = QueryParams::from(
        &format!("rid={}&limit={}&offset={}&beforeTime={}",
            id, limit, offset, beforeTime
        )
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/comment/hotwall/list")]
fn index_comment_hotwall_list(_req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/comment/hotwall/list/get";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/comment/like")]
fn index_comment_like(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = query.value("t").unwrap();
    let url = format!("https://music.163.com/weapi/v1/comment/{}", t);

    let _type = resource_type[ query.value("type").unwrap().parse::<usize>().unwrap_or(0) ];
    let threadId: String = if _type == "A_EV_2" {
        String::from(query.value("threadId").unwrap())
    } else {
        _type.to_owned() + query.value("id").unwrap()
    };
    let commentId = query.value("cid").unwrap();

    let info = QueryParams::from(
        &format!("threadId={}&commentId={}", threadId, commentId)
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/comment/music")]
fn index_comment_music(req: HttpRequest) ->impl Responder {
    let query = QueryParams::from(req.query_string());
    let rid = query.value("id").unwrap();
    let url = format!("https://music.163.com/api/v1/resource/comments/R_SO_4_{}", rid);

    let info = QueryParams::from(query).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/comment/mv")]
fn index_comment_mv(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let rid = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_MV_5_{}", rid);

    let info = QueryParams::from(query).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/comment/playlist")]
fn index_comment_playlist(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let rid = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/A_PL_0_{}", rid);

    let info = QueryParams::from( query ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/comment/video")]
fn index_comment_video(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let rid = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/v1/resource/comments/R_VI_62_{}", rid);

    let info = QueryParams::from( query ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/daily_signin")]
fn index_daily_signin(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/point/dailyTask";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/digitalAlbum/purchased")]
fn index_digitalAlbum_purchased(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/digitalAlbum/purchased";

    let query = QueryParams::from(req.query_string());
    let limit = query.value("limit").unwrap_or("30");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;

    let info = QueryParams::from(
        &format!("limit={}&offset={}&total={}",limit,offset,total)
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/banner")]
fn index_dj_banner() -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/banner/get";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/dj/category/excludehot")]
fn index_dj_category_exclude_hot() -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/category/excludehot";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/dj/category/recommend")]
fn index_dj_category_recommend() -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/home/category/recommend";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/dj/catelist")]
fn index_dj_category_list() -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/category/get";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/dj/detail")]
fn index_dj_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/get";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/hot")]
fn index_dj_hot(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/hot/v1";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/toplist/pay")]
fn index_dj_pay_gift(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/home/paygift/list?_nmclfl=1";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/program/detail")]
fn index_dj_program_details(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/dj/program/detail";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/program/toplist")]
fn index_dj_program_toplist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/program/toplist/v1";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/program/toplist/hours")]
fn index_dj_program_toplist_hours(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/djprogram/toplist/hours";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/program")]
fn index_dj_program(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/dj/program/byradio";

    let query = QueryParams::from(req.query_string());
    let radioId = query.value("rid").unwrap();
    let limit = query.value("limit").unwrap_or("30");
    let offset = query.value("offset").unwrap_or("0");
    let asc = query.value("asc").unwrap_or("false").parse::<bool>().unwrap_or(false);

    let info = QueryParams::from(
        &format!("radioId={}&limit={}&offset={}&asc={}",
            radioId, limit, offset, asc
        )
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/radio/hot")]
fn index_dj_radio_hot(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/djradio/hot";
    let info = QueryParams::from( req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/recommend")]
fn index_dj_recommend() -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/recommend/v1";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/dj/recommend/type")]
fn index_dj_recommend_type(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/recommend";
    let query = QueryParams::from(req.query_string());
    let cateId = query.value("type").unwrap_or("10001");
    let info = QueryParams::from(
        &format!("cateId={}", cateId)
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/sub")]
fn index_dj_sub(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = if query.value("t").unwrap_or("0").parse::<usize>().unwrap() == 1 {
        "sub"
    } else {
        "unsub"
    };
    let url = format!("https://music.163.com/weapi/djradio/{}", t);

    let info = QueryParams::from(
        &format!("id={}", query.value("rid").unwrap())
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/dj/sublist")]
fn index_dj_sub_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/get/subed";
    let info = QueryParams::from( req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/today/perfered")]
fn index_dj_today_perfered(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/djradio/home/today/perfered";
    let info = QueryParams::from( req.query_string()).json();
    HttpResponse::Ok().body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &info
        )
    )
}

#[get("/dj/toplist")]
fn index_dj_toplist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/djradio/toplist";

    let query = QueryParams::from(req.query_string());
    let limit = query.value("limit").unwrap_or("100");
    let offset = query.value("offset").unwrap_or("0");
    let rtype = if query.value("type").unwrap_or("new") == "new" {
        0
    } else {
        1
    };

    let info = QueryParams::from(
        &format!("limit={}&offset={}&type={}",
            limit, offset, rtype
        )
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/toplist/hours")]
fn index_dj_toplist_hours(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/dj/toplist/hours";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/toplist/newcomer")]
fn index_dj_toplist_newcomer(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/dj/toplist/newcomer";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/toplist/pay")]
fn index_dj_toplist_pay(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/djradio/toplist/pay";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/dj/toplist/popular")]
fn index_dj_toplist_popular(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/dj/toplist/popular";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/event")]
fn index_event(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/event/get";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/event/del")]
fn index_event_del(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/eapi/event/delete";
    let query = QueryParams::from(req.query_string());
    let info = QueryParams::from(
        &format!("id={}", query.value("evId").unwrap())
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/event/forward")]
fn index_event_forward(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/event/forward";
    let query = QueryParams::from(req.query_string());
    let info = QueryParams::from(
        &format!("forwards={}&id={}&eventUserId={}",
            query.value("forwards").unwrap(),
            query.value("evId").unwrap(),
            query.value("uid").unwrap()
        )
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/fm_trash")]
fn index_fm_trash(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let time = query.value("time").unwrap_or("25");
    let url = format!("https://music.163.com/weapi/radio/trash/add?alg=RT&songId={}&time={}", id, time);

    let info = QueryParams::from(
        &format!("songId={}", id)
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/follow")]
fn index_follow(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = if query.value("t").unwrap_or("0") == "1" {
        "follow"
    } else {
        "delfollow"
    };
    let id = query.value("id").unwrap();
    let url = format!("https://music.163.com/weapi/user/{}/{}",t,id);
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &"{}"
            )
        )
}

#[get("/hot/topic")]
fn index_hot_topic(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/act/hot";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/like")]
fn index_like(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let alg = query.value("alg").unwrap_or("itembased");
    let id = query.value("id").unwrap();
    let time = query.value("time").unwrap_or("25");
    let url = format!(
        "https://music.163.com/weapi/radio/like?alg={}&trackId={}&time={}",
        alg, id, time
    );
    let info = QueryParams::from(
        &format!("trackId={}&like={}",
            id, query.value("like").unwrap_or("true")
        )
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/likelist")]
fn index_likelist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/song/like/get";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/login")]
fn index_login(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/login";
    let query = QueryParams::from(req.query_string());
    let username = query.value("email").unwrap();
    let password = Crypto::hash_encrypt(
        query.value("password").unwrap(),
        HashType::md5,
        hex::encode
    );
    let rememberLogin = query.value("rememberLogin").unwrap_or("true");

    let info = QueryParams::from(
        &format!("username={}&password={}&rememberLogin={}",
                 username, password, rememberLogin
        )
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "pc",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/login/cellphone")]
fn index_login_cellphone(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/login/cellphone";
    let query = QueryParams::from(req.query_string());
    let phone = query.value("phone").unwrap();
    let countrycode = query.value("countrycode").unwrap();
    let password = Crypto::hash_encrypt(
        query.value("password").unwrap(),
        HashType::md5,
        hex::encode
    );
    let rememberLogin = query.value("rememberLogin").unwrap_or("true");

    let info = QueryParams::from(
        &format!("phone={}&countrycode={}&password={}&rememberLogin={}",
                 phone, countrycode, password, rememberLogin
        )
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "pc",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/login/refresh")]
fn index_login_refresh() -> impl Responder {
    let url = "https://music.163.com/weapi/login/token/refresh";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "pc",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/login/status")]
fn index_login_status(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "GET",
                "",
                "",
                url,
                &"{}"
            )
        )
}

#[get("/logout")]
fn index_logout(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/logout";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &"{}"
            )
        )
}

#[get("/lyric")]
fn index_lyric(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/song/lyric?lv=-1&kv=-1&tv=-1";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "linuxapi",
                url,
                &info
            )
        )
}

#[get("/msg/comments")]
fn index_msg_comments(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let uid = query.value("uid").unwrap();
    let url = format!("https://music.163.com/api/v1/user/comments/{})",uid);
    let beforeTime = query.value("before").unwrap_or("-1");
    let limit = query.value("limit").unwrap_or("30");
    let total = true;
    let info = QueryParams::from(
        &format!("beforeTime={}&limit={}&total={}&uid={}",
            beforeTime, limit, total, uid
        )
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/msg/forwards")]
fn index_msg_forwards(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/forwards/get";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/msg/notices")]
fn index_msg_notices(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/msg/notices";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/msg/private")]
fn index_msg_private(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/msg/private/users";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/msg/private/history")]
fn index_msg_private_history(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/msg/private/history";
    let info = QueryParams::from(req.query_string())
            .replace_key("uid", "userId")
            .replace_key("before", "time")
            .json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/mv/all")]
fn index_mv_all(req: HttpRequest) -> impl Responder {
    let url = "https://interface.music.163.com/api/mv/all";

    let query = QueryParams::from(req.query_string());
    let area = query.value("area").unwrap_or("全部");
    let rtype = query.value("type").unwrap_or("全部");
    let order = query.value("order").unwrap_or("上升最快");
    let tags_value = format!(r#"{{"地区":"{}","类型":"{}","排序":"{}"}}"#,
        area, rtype, order
    );
    let tags = format!(r#"{{"tags":{}}}"#, tags_value);
    let limit = query.value("limit").unwrap_or("30");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;

    let info = QueryParams::from(
        &format!("tags={}&offset={}&total={}&limit={}",
            tags, offset, total, limit
        )
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/mv/detail")]
fn index_mv_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/mv/detail";
    let info = QueryParams::from(req.query_string())
        .replace_key("mvid", "id")
        .json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/mv/exclusive/rcmd")]
fn index_mv_exclusive_rcmd(req: HttpRequest) -> impl Responder {
    let url = "https://interface.music.163.com/api/mv/exclusive/rcmd";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/mv/first")]
fn index_mv_first(req: HttpRequest) -> impl Responder {
    let url = "https://interface.music.163.com/weapi/mv/first";
    let info = QueryParams::from(req.query_string()).json();
    println!("info={}", info);
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/mv/sub")]
fn index_mv_sub(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = if query.value("t").unwrap_or("0") == "1" {
        "sub"
    } else {
        "unsub"
    };
    let url = format!("https://music.163.com/weapi/mv/{}", t);
    let id = query.value("mvid").unwrap();
    let info = query.replace_key("mvid", "mvIds")
        .replace_value(id, &format!("[{}]", id))
        .json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/mv/sublist")]
fn index_mv_sublist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/cloudvideo/allvideo/sublist";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/mv/url")]
fn index_mv_url(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/song/enhance/play/mv/url";

    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let r = query.value("res").unwrap_or("1080");

    let info = QueryParams::from(
        &format!("id={}&r={}", id, r)
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/personal_fm")]
fn index_personal_fm(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/radio/get";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/personalized")]
fn index_personalized(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/playlist";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/personalized/djprogram")]
fn index_personalized_djprogram(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/djprogram";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/personalized/mv")]
fn index_personalized_mv(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/mv";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/personalized/newsong")]
fn index_personalized_newsong(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/newsong";
    let info = QueryParams::from(
        &format!("type=recommend")
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/personalized/privatecontent")]
fn index_personalized_privatecontent(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/personalized/privatecontent";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/playlist/catlist")]
fn index_playlist_catlist() -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/catalogue";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/playlist/create")]
fn index_playlist_create(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/create";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/playlist/delete")]
fn index_playlist_delete(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/delete";
    let info = QueryParams::from(req.query_string())
        .replace_key("id","pid")
        .json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/playlist/desc/update")]
fn index_playlist_desc_update(req: HttpRequest) -> impl Responder {
    let url = "http://interface3.music.163.com/eapi/playlist/desc/update";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "eapi",
                url,
                &info
            )
        )
}

#[get("/playlist/detail")]
fn index_playlist_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v3/playlist/detail";

    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let s = query.value("s").unwrap_or("8");
    let info = QueryParams::from(
        &format!("id={}&n=100000&s={}", id, s)
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "linuxapi",
                url,
                &info
            )
        )
}

#[get("/playlist/hot")]
fn index_playlist_hot() -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/hottags";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/playlist/name/update")]
fn index_playlist_name_update(req: HttpRequest) -> impl Responder {
    let url = "http://interface3.music.163.com/eapi/playlist/update/name";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "eapi",
                url,
                &info
            )
        )
}

#[get("/playlist/subscribe")]
fn index_playlist_subscribe(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = if query.value("t").unwrap_or("0") == "1" {
        "subscribe"
    } else {
        "unsubscribe"
    };
    let url = format!("https://music.163.com/weapi/playlist/{}", t);
    let info = QueryParams::from(
        &format!("id={}", query.value("id").unwrap())
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/playlist/subscribers")]
fn index_playlist_subscribers(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/subscribers";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "eapi",
                url,
                &info
            )
        )

}

#[get("/playlist/tags/update")]
fn index_playlist_tags_update(req: HttpRequest) -> impl Responder {
    let url = "http://interface3.music.163.com/eapi/playlist/tags/update";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "eapi",
                url,
                &info
            )
        )
}

#[get("/playlist/tracks")]
fn index_playlist_tracks(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/manipulate/tracks";
    let query = QueryParams::from(req.query_string());
    let info = QueryParams::from(
        &format!("op={}&pid={}&trackIds=[{}]",
            query.value("op").unwrap(),
            query.value("pid").unwrap(),
            query.value("tracks").unwrap(),
        )
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "eapi",
                url,
                &info
            )
        )
}

#[get("/playlist/update")]
fn index_playlist_update(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/batch";

    let query = QueryParams::from(req.query_string());
    let desc = query.value("desc").unwrap_or("");
    let tags = query.value("tags").unwrap_or("");
    let id = query.value("id").unwrap();
    let name = query.value("name").unwrap();

    let desc_update = format!(r#"{{"id":{},"desc":"{}"}}"#, id, desc);
    let tags_update = format!(r#"{{"id":{},"tags":"{}"}}"#, id, tags);
    let name_update = format!(r#"{{"id":{},"name":"{}"}}"#, id, name);
    let info = QueryParams::from(
        &format!(r#"/api/playlist/desc/update={}&/api/playlist/tags/update={}&/api/playlist/update/name={}"#,
            desc_update, tags_update, name_update
        )
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/playmode/intelligence/list")]
fn index_playmode_interlligence_list(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/playmode/intelligence/list";

    let query = QueryParams::from(req.query_string());
    let songId = query.value("id").unwrap();
    let _type = "fromPlayOne";
    let playlistId = query.value("pid").unwrap();
    let startMusicId = if let Some(sId) = query.value("sid") {
        sId
    } else {
        songId
    };
    let count = query.value("count").unwrap_or("1");

    let info = QueryParams::from(
        &format!("songId={}&type={}&playlistId={}&startMusicId={}&count={}",
            songId, _type, playlistId, startMusicId, count
        )
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/program/recommend")]
fn index_program_recommend(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/program/recommend/v1";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/rebind")]
fn index_rebind(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/user/replaceCellphone";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/recommend/resource")]
fn index_recommend_resource(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/recommend/resource";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/recommend/songs")]
fn index_recommend_songs() -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/recommend/songs";
    let info = format!(r#"{{"limit":"{}","offset":"{}","total":"{}"}}"#,
        20, 0, true
    );
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/register/cellphone")]
fn index_register_cellphone(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/register/cellphone";
    let query = QueryParams::from(req.query_string());
    let pw = query.value("password").unwrap();
    let info = query.replace_value(
            pw,
            &Crypto::hash_encrypt(pw, HashType::md5, hex::encode)
        ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/related/allvideo")]
fn index_related_allvideo(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/cloudvideo/v1/allvideo/rcmd";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/related/playlist")]
fn index_related_playlist(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let url = format!("https://music.163.com/playlist?id={}", id);
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "pc",
                "weapi",
                &url,
                &"{}"
            )
        )
}

#[get("/resource/like")]
fn index_resource_like(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let t = if query.value("t").unwrap_or("0") == "1" {
        "like"
    } else {
        "unlike"
    };
    let url = format!("https://music.163.com/weapi/resource/{}", t);

    let rtype = resource_type[
        query.value("t").unwrap().parse::<usize>().unwrap_or(0)
        ];

    let threadId = if rtype == "A_EV_2_" {
        String::from(query.value("threadId").unwrap())
    } else {
        rtype.to_owned() + query.value("id").unwrap()
    };

    let info = QueryParams::from(
        &format!("threadId={}", threadId)
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "pc",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/scrobble")]
fn index_scrobble(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/feedback/weblog";

    let query = QueryParams::from(req.query_string());
    let id = query.value("id").unwrap();
    let sourceId = query.value("sourceid").unwrap();
    let time = query.value("time").unwrap();

    let json_value = format!(
        r#"{{"download":"0","end":"playend","id":"{}","sourceId":"{}","time":"{}","type":"song","wifi":"0"}}"#,
        id,sourceId,time
    );
    let log_value = format!(r#"[{{"action":"play","json":"{}"}}]"#,
        json_value
    );

    let info = format!(r#"{{"logs":"{}"}}"#, log_value);
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "pc",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/search")]
fn index_search(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/search/get";

    let query = QueryParams::from(req.query_string());
    let s = query.value("keywords").unwrap();
    let rtype = query.value("type").unwrap_or("1");
    let limit = query.value("limit").unwrap_or("30");
    let offset = query.value("offset").unwrap_or("0");

    let info = QueryParams::from(
        &format!("s={}&type={}&limit={}&offset={}",
             s, rtype, limit, offset
        )
    ).json();
    println!("info={}", info);

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/search/default")]
fn index_search_default(req: HttpRequest) -> impl Responder{
    let url = "http://interface3.music.163.com/eapi/search/defaultkeyword/get";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "eapi",
                url,
                &"{}"
            )
        )
}

#[get("/search/hot")]
fn index_search_hot() -> impl Responder {
    let url = "https://music.163.com/weapi/search/hot";
    let value = r#"{{"type":1111}}"#;
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &value
            )
        )
}

#[get("/search/hot/detail")]
fn index_search_hot_detail(req: HttpRequest) -> impl Responder{
    let url = "https://music.163.com/weapi/hotsearchlist/get";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/search/multimatch")]
fn index_search_multimatch(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/search/suggest/multimatch";
    let info = QueryParams::from(req.query_string())
        .replace_key("keywords","s")
        .json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/search/suggest")]
fn index_search_suggest(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let rtype = if query.value("type").unwrap_or("web") == "mobile" {
        "keyword"
    } else {
        "web"
    };

    let url = format!("https://music.163.com/weapi/search/suggest/{}", rtype);
    let s = query.value("keywords").unwrap();
    let info = QueryParams::from(
        &format!("s={}", s)
    ).json();
    println!("info:{}", info);
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/send/playlist")]
fn index_send_playlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/msg/private/send";

    let query = QueryParams::from(req.query_string());
    let id = query.value("playlist").unwrap();
    let msg = query.value("msg").unwrap();
    let userIds = query.value("user_ids").unwrap();

    let info = QueryParams::from(
        &format!("id={}&type=playlist&msg={}&userIds=[{}]",id,msg,userIds)
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/send/text")]
fn index_send_text(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/msg/private/send";

    let query = QueryParams::from(req.query_string());
    let id = query.value("playlist").unwrap();
    let msg = query.value("msg").unwrap();
    let userIds = query.value("user_ids").unwrap();

    let info = QueryParams::from(
        &format!("id={}&type=text&msg={}&userIds=[{}]",id,msg,userIds)
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/setting")]
fn index_setting(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/user/setting";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &"{}"
            )
        )
}

#[get("/share/resource")]
fn index_share_resource(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/share/friends/resource";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/simi/artist")]
fn index_simi_artist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/discovery/simiArtist";
    let info = QueryParams::from(req.query_string())
        .replace_key("id", "artistid")
        .json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/simi/mv")]
fn index_simi_mv(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/discovery/simiMV";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/simi/playlist")]
fn index_simi_playlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/discovery/simiPlaylist";
    let info = QueryParams::from(req.query_string())
        .replace_key("id", "songid")
        .json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/simi/song")]
fn index_simi_song(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/simiSong";
    let info = QueryParams::from(req.query_string())
        .replace_key("id", "songid")
        .json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/simi/user")]
fn index_simi_user(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/discovery/simiUser";
    let info = QueryParams::from(req.query_string())
        .replace_key("id", "songid")
        .json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/song/detail")]
fn index_song_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v3/song/detail";
    let query = QueryParams::from(req.query_string());
//    let ids: Vec<usize> = query.value("ids")
//        .unwrap().split(",").collect::<Vec<usize>>();
//
//    let info = QueryParams::from(
//        &format!("c=[{}]&ids=[]",
//            ids.iter().map(|id| {
//                format!(r#"{{"id":"{}"}}"#, id)
//            }
//            ).collect(),
//            //ids
//        )
//    );
    let info = "hello";
    println!("info:{}", info);
    
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/song/url")]
fn index_song_url(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/song/enhance/player/url";
    let query = QueryParams::from(req.query_string())
        .replace_key("id", "ids");
    let br = query.value("br").unwrap_or("&br=999000");
    let ids = query.value("ids").unwrap();
    let value = query.replace_value(
        ids,
        &format!("[{}]", ids)
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
        api::create_request(
            "POST",
            "",
            "linuxapi",
            url,
            &value
        )
    )
}

#[get("/top/album")]
fn index_top_album(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/album/new";

    let query = QueryParams::from(req.query_string());
    let area = query.value("type").unwrap_or("ALL");
    let limit = query.value("limit").unwrap_or("50");
    let offset = query.value("offset").unwrap_or("0");
    let total = true;

    let info = QueryParams::from(
        &format!("area={}&limit={}&offset={}&total={}",
            area, limit, offset, total
        )
    ).json();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
        api::create_request(
            "POST",
            "",
            "weapi",
            url,
            &info
        )
    )
}

#[get("/top/artists")]
fn index_top_artists(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/artist/top";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/top/list")]
fn index_top_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v3/playlist/detail";
    let query = QueryParams::from(req.query_string());
    let id = topList[query.value("idx").unwrap_or("0").parse::<usize>().unwrap()];
    let info = QueryParams::from(
        &format!("id={}&n=10000", id)
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "Post",
                "",
                "linuxapi",
                url,
                &info
            )
        )
}

#[get("/top/mv")]
fn index_top_mv(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/mv/toplist";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/top/playlist")]
fn index_top_playlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/list";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/top/playlist/highquality")]
fn index_top_playlist_highquality(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/playlist/highquality/list";

    let query = QueryParams::from(req.query_string());
    let cat = query.value("cat").unwrap_or("全部");
    let limit = query.value("limit").unwrap_or("50");
    let lasttime = query.value("before").unwrap_or("0");
    let total = true;

    let info = QueryParams::from(
        &format!("cat={}&limit={}&lasttime={}&total={}", cat, limit, lasttime, total)
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .json(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/top/song")]
fn index_top_song(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/discovery/new/songs";
    let info = QueryParams::from(
        &format!("areaId={}&total=true", QueryParams::from(req.query_string()).value("type").unwrap_or("0"))
    ).json();
    HttpResponse::Ok()
        .content_type(
            CONTENT_TP
        )
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/toplist")]
fn index_toplist() -> impl Responder {
    let url = "https://music.163.com/weapi/toplist";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &"{}"
            )
        )
}

#[get("/toplist/artist")]
fn index_toplist_artist() -> impl Responder {
    let url = "https://music.163.com/weapi/toplist/artist";
    let info = format!(r#"{{"type":"1","limit":"100","offset":"0","total":"true"}}"#);
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/toplist/detail")]
fn index_toplist_detail() -> impl Responder {
    let url = "https://music.163.com/weapi/toplist/detail";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "linuxapi",
                url,
                &"{}"
            )
        )
}

#[get("/user/audio")]
fn index_user_audio(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/djradio/get/byuser";
    let info = QueryParams::from(req.query_string())
        .replace_key("uid", "userId")
        .json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}


#[get("/user/cloud")]
fn index_user_cloud(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/cloud/get";
    let info = QueryParams::from(req.query_string())
        .json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("user/cloud/del")]
fn index_user_cloud_del(req: HttpRequest) -> impl Responder {
    let url = "http://music.163.com/weapi/cloud/del";
    let query = QueryParams::from(req.query_string())
        .replace_key("id", "songIds");
    let songIds = query.value("songIds").unwrap();
    let info = query.replace_value(songIds, &format!("[{}]", songIds))
        .json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/user/cloud/detail")]
fn index_user_cloud_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/cloud/get/byids";
    let query = QueryParams::from(req.query_string());
    let queryValue = query.value("id").unwrap();

    let info = serde_json::json!({
        "songIds": queryValue.split(',').collect::<Vec<&str>>()
    }).to_string();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                url,
                &info
            )
        )
}

#[get("/user/detail")]
fn index_user_detail(req: HttpRequest) -> impl Responder {
    let url = format!("https://music.163.com/weapi/v1/user/detail/{}",
        QueryParams::from(req.query_string()).value("uid").unwrap()
    );
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &"{}"
            )
        )
}


#[get("/user/dj")]
fn index_user_dj(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let url = format!("https://music.163.com/weapi/dj/program/{}",
        query.value("uid").unwrap()
    );

    let info = serde_json::json!({
        "limit": query.value("limit").unwrap_or("30"),
        "offset": query.value("offset").unwrap_or("0")
    }).to_string();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/user/event")]
fn index_user_event(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let url = format!("https://music.163.com/weapi/event/get/{}",
        query.value("uid").unwrap()
    );

    let info = serde_json::json!({
        "getcounts": "true",
        "time": query.value("lasttime").unwrap_or("-1"),
        "limit": query.value("limit").unwrap_or("30"),
        "total": "true"
    }).to_string();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/user/followeds")]
fn index_user_followeds(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let uid = query.value("uid").unwrap();
    let url = format!("https://music.163.com/eapi/user/getfolloweds/{}",
        uid
    );

    let info = serde_json::json!({
        "userId": uid,
        "time": query.value("lasttime").unwrap_or("-1"),
        "limit": query.value("limit").unwrap_or("30")
    }).to_string();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/user/follows")]
fn index_user_follows(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let url = format!("https://music.163.com/weapi/user/getfollows/{}",
        query.value("uid").unwrap()
    );

    let info = serde_json::json!({
        "offset": query.value("offset").unwrap_or("0"),
        "limit": query.value("limit").unwrap_or("30"),
        "order": "true"
    }).to_string();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/user/playlist")]
fn index_user_playlist(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/user/playlist";
    let info = QueryParams::from(
        QueryParams::from(req.query_string())
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/user/record")]
fn index_user_record(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/v1/play/record";
    let info = QueryParams::from(
        QueryParams::from(req.query_string())
    ).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )

}

#[get("/user/subcount")]
fn index_user_subcount(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/subcount";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &"{}"
            )
        )
}

#[get("/user/update")]
fn index_user_update(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/user/profile/update";
    let query = QueryParams::from(req.query_string());
    let info = serde_json::json!({
        "avatarImgId": "0",
        "birthday": query.value("birthday").unwrap(),
        "city": query.value("city").unwrap(),
        "gender": query.value("gender").unwrap(),
        "nickname": query.value("nickname").unwrap(),
        "province": query.value("province").unwrap(),
        "signature": query.value("signature").unwrap()
    }).to_string();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/video/detail")]
fn index_video_detail(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/cloudvideo/v1/video/detail";
    let info = QueryParams::from(req.query_string()).json();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/video/group")]
fn index_video_group(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/videotimeline/videogroup/get";
    let query = QueryParams::from(req.query_string());
    let info = serde_json::json!({
        "groupId": query.value("id").unwrap(),
        "offset": query.value("offset").unwrap_or("0"),
        "needUrl": true,
        "resolution": query.value("res").unwrap_or("1080")
    }).to_string();
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/video/group/list")]
fn index_video_group_list(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/api/cloudvideo/group/list";
    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &"{}"
            )
        )
}

#[get("/video/sub")]
fn index_video_sub(req: HttpRequest) -> impl Responder {
    let query = QueryParams::from(req.query_string());
    let url = format!("https://music.163.com/weapi/cloudvideo/video/{}",
        query.value("t").unwrap()
    );
    let info = serde_json::json!({
        "id": query.value("id").unwrap()
    }).to_string();

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

#[get("/video/url")]
fn index_video_url(req: HttpRequest) -> impl Responder {
    let url = "https://music.163.com/weapi/cloudvideo/playurl";
    let query = QueryParams::from(req.query_string());
    let info = serde_json::json!({
        "ids": format!("{:?}",
            query.value("id").unwrap().split(',').collect::<Vec<&str>>()),
        "resolution": query.value("res").unwrap_or("1080")
    }).to_string();

    println!("info={}", info);

    HttpResponse::Ok()
        .content_type(CONTENT_TP)
        .body(
            api::create_request(
                "POST",
                "",
                "weapi",
                &url,
                &info
            )
        )
}

pub fn start_server() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index_activate_init_profile)
            .service(index_album)
            .service(index_album_detail_dynamic)
            .service(index_album_newest)
            .service(index_album_sub)
            .service(index_album_sublist)
            .service(index_artist_album)
            .service(index_artist_desc)
            .service(index_artist_list)
            .service(index_artist_mv)
            .service(index_artist_sub)
            .service(index_artist_sublist)
            .service(index_artist_top_song)
            .service(index_banner)
            .service(index_batch)
            .service(index_captcha_register)
            .service(index_captcha_sent)
            .service(index_captcha_verify)
            .service(index_cellphone_existence_check)
            .service(index_check_music)
            .service(index_comment)
            .service(index_comment_album)
            .service(index_comment_dj)
            .service(index_comment_event)
            .service(index_comment_hot)
            .service(index_comment_hotwall_list)
            .service(index_comment_like)
            .service(index_comment_music)
            .service(index_comment_mv)
            .service(index_comment_playlist)
            .service(index_comment_video)
            .service(index_daily_signin)
            .service(index_digitalAlbum_purchased)
            .service(index_dj_banner)
            .service(index_dj_category_exclude_hot)
            .service(index_dj_category_recommend)
            .service(index_dj_category_list)
            .service(index_dj_detail)
            .service(index_dj_hot)
            .service(index_dj_pay_gift)
            .service(index_dj_program)
            .service(index_dj_program_details)
            .service(index_dj_program_toplist)
            .service(index_dj_program_toplist_hours)
            .service(index_dj_radio_hot)
            .service(index_dj_recommend)
            .service(index_dj_recommend_type)
            .service(index_dj_sub)
            .service(index_dj_sub_list)
            .service(index_dj_today_perfered)
            .service(index_dj_toplist)
            .service(index_dj_toplist_hours)
            .service(index_dj_toplist_newcomer)
            .service(index_dj_toplist_pay)
            .service(index_dj_toplist_popular)
            .service(index_event)
            .service(index_event_del)
            .service(index_event_forward)
            .service(index_fm_trash)
            .service(index_follow)
            .service(index_hot_topic)
            .service(index_like)
            .service(index_likelist)
            .service(index_login)
            .service(index_login_cellphone)
            .service(index_login_refresh)
            .service(index_login_status)
            .service(index_logout)
            .service(index_lyric)
            .service(index_msg_comments)
            .service(index_msg_forwards)
            .service(index_msg_notices)
            .service(index_msg_private)
            .service(index_msg_private_history)
            .service(index_mv_all)
            .service(index_mv_detail)
            .service(index_mv_exclusive_rcmd)
            .service(index_mv_first)
            .service(index_mv_sub)
            .service(index_mv_sublist)
            .service(index_mv_url)
            .service(index_personal_fm)
            .service(index_personalized)
            .service(index_personalized_djprogram)
            .service(index_personalized_mv)
            .service(index_personalized_newsong)
            .service(index_personalized_privatecontent)
            .service(index_playlist_catlist)
            .service(index_playlist_create)
            .service(index_playlist_delete)
            .service(index_playlist_desc_update)
            .service(index_playlist_detail)
            .service(index_playlist_hot)
            .service(index_playlist_name_update)
            .service(index_playlist_subscribe)
            .service(index_playlist_subscribers)
            .service(index_playlist_tags_update)
            .service(index_playlist_tracks)
            .service(index_playlist_update)
            .service(index_playmode_interlligence_list)
            .service(index_program_recommend)
            .service(index_rebind)
            .service(index_recommend_resource)
            .service(index_recommend_songs)
            .service(index_register_cellphone)
            .service(index_related_allvideo)
            .service(index_related_playlist)
            .service(index_resource_like)
            .service(index_scrobble)
            .service(index_search)
            .service(index_search_default)
            .service(index_search_hot)
            .service(index_search_hot_detail)
            .service(index_search_multimatch)
            .service(index_search_suggest)
            .service(index_send_playlist)
            .service(index_send_text)
            .service(index_setting)
            .service(index_share_resource)
            .service(index_simi_artist)
            .service(index_simi_mv)
            .service(index_simi_playlist)
            .service(index_simi_song)
            .service(index_simi_user)
            .service(index_song_detail)
            .service(index_song_url)
            .service(index_top_album)
            .service(index_top_artists)
            .service(index_top_list)
            .service(index_top_mv)
            .service(index_top_playlist)
            .service(index_top_playlist_highquality)
            .service(index_top_song)
            .service(index_toplist)
            .service(index_toplist_artist)
            .service(index_toplist_detail)
            .service(index_user_audio)
            .service(index_user_cloud)
            .service(index_user_cloud_del)
            .service(index_user_cloud_detail)
            .service(index_user_detail)
            .service(index_user_dj)
            .service(index_user_event)
            .service(index_user_followeds)
            .service(index_user_follows)
            .service(index_user_playlist)
            .service(index_user_record)
            .service(index_user_subcount)
            .service(index_user_update)
            .service(index_video_detail)
            .service(index_video_group)
            .service(index_video_group_list)
            .service(index_video_sub)
            .service(index_video_url)
            .route("/", web::get().to(index_root))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        dbg!("server runing @ http://127.0.0.1:8000");
        server.bind("127.0.0.1:8000").unwrap()
    };

    server.run().unwrap();
}

#[cfg(test)]
mod tests {

    use serde_json::Serializer;
    use serde_json::json;
    use urlqstring::QueryParams;

    #[test]
    fn it_works() {
        let str = "id=5374627,5374628,5374629";
        let value = QueryParams::from(str).value("id").unwrap();
        let value: Vec<&str> = value.split(',').collect();

        let s1 = format!("{:?}", value);
        println!("s1={}", s1);

        let s = json!({
            "songIds":s1
        });

        println!("s={}", s);
    }
}