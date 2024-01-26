use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use warp::{http::Response, Filter, Rejection};

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin() // 或者使用 .allow_origin("http://example.com") 指定具体的域
        .allow_headers(vec!["Content-Type"]) // 指定允许的头部
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]); // 指定允许的方法
    let font_route = warp::path("font")
        .and(warp::query::<HashMap<String, String>>())
        .and_then(serve_font)
        .with(cors);

    warp::serve(font_route).run(([127, 0, 0, 1], 3030)).await;
}

async fn serve_font(params: HashMap<String, String>) -> Result<impl warp::Reply, Rejection> {
    if let Some(path) = params.get("path") {
        if !is_valid_path(path) {
            return Ok(Response::builder()
                .status(400)
                .body("Invalid file path".into()));
        }

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Ok(Response::builder()
                    .status(404)
                    .body("File not found".into()))
            }
        };

        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();

        let content_type = determine_font_mime_type(path);

        Ok(Response::builder()
            .header("Content-Type", content_type)
            .body(contents))
    } else {
        Ok(Response::builder()
            .status(400)
            .body("Missing file path".into()))
    }
}

fn is_valid_path(path: &str) -> bool {
    let path = Path::new(path);
    let allowed_extensions = ["ttf", "otf", "woff", "woff2", "ttc"];

    // 检查是否是绝对路径
    if !path.is_absolute() {
        return false;
    }

    // 检查文件扩展名是否是允许的字体类型
    return path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| allowed_extensions.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false);
}

fn determine_font_mime_type(path: &str) -> &'static str {
    match Path::new(path).extension().and_then(|ext| ext.to_str()) {
        Some("ttf") => "font/ttf",
        Some("otf") => "font/otf",
        Some("woff") => "application/font-woff",
        Some("woff2") => "application/font-woff2",
        Some("ttc") => "font/collection", // 注意这不是标准的 MIME 类型
        _ => "application/octet-stream",  // 未知文件类型
    }
}
