use axum::routing::get;
use axum::handler::Handler;
// use axum::http::StatusCode;
// use axum::response::AppendHeaders;

// #[warn(unused_imports)]

#[tokio::main]
pub async fn main() {
     // Build our application by creating our router.
     let app = axum::Router::new()
     .fallback(
        fallback.into_service()
    )
     .route("/",
         get(hello)
     )
     .route("/html", 
        get(get_demo_html)
    )
    .route("/demo_status", 
        get(demo_status)
    )
    .route("/demo_uri", 
        get(demo_uri)
    )
    .route("/demo.png", 
        get(get_demo_png)
    );

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn hello() -> String {
    "Hello, World!".into()
 }

 pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

pub async fn get_demo_html() -> axum::response::Html<&'static str> {
    include_str!("./webpages/hello_world.html").into()
}

pub async fn demo_status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Response status is OK".to_string())
}

pub async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}

pub async fn get_demo_png() -> impl axum::response::IntoResponse {
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );
    (
        axum::response::Headers([
            (axum::http::header::CONTENT_TYPE, "image/png"),
        ]),
        base64::decode(png).unwrap(),
    )
}

