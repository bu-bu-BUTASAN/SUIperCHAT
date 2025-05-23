//! WebSocketサーバールートハンドラー
//!
//! WebSocketおよびOBSのHTTPルートハンドラーを提供します。

use actix_web::{get, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

/// ## WebSocket ルートハンドラー
///
/// WebSocket 接続リクエストを処理し、`WsSession` アクターを開始します。
///
/// ### Arguments
/// - `req`: HTTPリクエスト (`HttpRequest`)
/// - `stream`: ペイロードストリーム (`actix_web::web::Payload`)
///
/// ### Returns
/// - `Result<HttpResponse, Error>`: WebSocket ハンドシェイク応答 or エラー
#[get("/ws")]
pub async fn websocket_route(
    req: HttpRequest,
    stream: actix_web::web::Payload,
) -> Result<HttpResponse, Error> {
    println!("Received websocket upgrade request");
    ws::start(
        crate::ws_server::create_ws_session(req.clone()),
        &req,
        stream,
    )
}

/// ## OBSステータスページハンドラー
///
/// OBS用のステータス情報ページを提供するハンドラー
///
/// ### Returns
/// - `HttpResponse`: HTML形式のステータスページ
#[get("/status")]
pub async fn status_page() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../src/static/obs/status.html"))
}

/// ## OBSインデックスページハンドラー
///
/// OBS用のメインHTMLページを提供するハンドラー
///
/// ### Returns
/// - `HttpResponse`: HTML形式のインデックスページ
#[get("/obs")]
pub async fn obs_index_page() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../src/static/obs/index.html"))
}

/// ## OBSスタイルシートハンドラー
///
/// OBS用のCSSファイルを提供するハンドラー
///
/// ### Returns
/// - `HttpResponse`: CSS形式のスタイルシート
#[get("/obs/styles.css")]
pub async fn obs_styles() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(include_str!("../../src/static/obs/styles.css"))
}

/// ## OBSスクリプトハンドラー
///
/// OBS用のJavaScriptファイルを提供するハンドラー
///
/// ### Returns
/// - `HttpResponse`: JavaScript形式のスクリプト
#[get("/obs/script.js")]
pub async fn obs_script() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/javascript; charset=utf-8")
        .body(include_str!("../../src/static/obs/script.js"))
}
