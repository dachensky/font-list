use font_kit::handle::Handle;
use font_kit::source::SystemSource;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use ttf_parser::Face;
use warp::{http::Response, Filter, Rejection};

#[derive(Serialize)]
struct FontData {
    post_script: Option<String>,
    file_name: String,
    font_name: String,
    family_name: Vec<FamilyName>,
    weight: u16,
    path: String,
}

#[derive(Serialize)]
struct FamilyName {
    font_family: Option<String>,
    language: String,
    style: Option<String>,
}

fn get_system_fonts() -> Result<Vec<FontData>, String> {
    let source = SystemSource::new();
    let mut fonts_info: Vec<_> = Vec::new();
    let mut seen_fonts: HashSet<String> = HashSet::new();
    match source.all_fonts() {
        Ok(handles) => {
            for handle in handles {
                let path = match &handle {
                    Handle::Path { path, .. } => path.to_string_lossy().into_owned(),
                    Handle::Memory { .. } => String::from("Memory-Loaded Font"),
                };
                // 去除重复的字体
                if seen_fonts.contains(&path) {
                    continue;
                }
                seen_fonts.insert(path.clone());

                let font = match &handle {
                    Handle::Path { path, font_index } => {
                        font_kit::font::Font::from_path(path, *font_index).unwrap()
                    }
                    Handle::Memory { bytes, font_index } => {
                        font_kit::font::Font::from_bytes(bytes.clone(), *font_index).unwrap()
                    }
                };

                // let properties = font.properties();
                let full_name = font.full_name();

                let file_name = match &handle {
                    Handle::Path { path, .. } => path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned(),
                    Handle::Memory { .. } => String::from("Memory-Loaded Font"),
                };

                let data = fs::read(path).unwrap();

                let face = Face::parse(&data, 0).unwrap();

                let mut english_family_name: Option<String> = None;
                let mut chinese_family_name: Option<String> = None;
                let mut style: Option<String> = None;
                for name in face.names() {
                    if name.is_unicode() {
                        let language = name.language().primary_language();
                        if name.name_id == ttf_parser::name_id::TYPOGRAPHIC_FAMILY
                            || name.name_id == ttf_parser::name_id::FAMILY
                        {
                            match language {
                                "English" => english_family_name = name.to_string(),
                                "Chinese" => chinese_family_name = name.to_string(),
                                _ => (),
                            }
                        } else if name.name_id == ttf_parser::name_id::SUBFAMILY {
                            style = name.to_string();
                        }
                    }
                }

                let mut font_infos = Vec::new();
                if let Some(english_name) = english_family_name {
                    font_infos.push(FamilyName {
                        font_family: Some(english_name),
                        language: "English".to_string(),
                        style: style.clone(),
                    });
                }

                if let Some(chinese_name) = chinese_family_name {
                    font_infos.push(FamilyName {
                        font_family: Some(chinese_name),
                        language: "Chinese".to_string(),
                        style: style.clone(),
                    });
                }

                let font_data = FontData {
                    post_script: font.postscript_name(),
                    file_name,
                    font_name: full_name,
                    family_name: font_infos,
                    weight: face.weight().to_number(),
                    path: match &handle {
                        Handle::Path { path, .. } => path.to_string_lossy().into_owned(),
                        Handle::Memory { .. } => String::from("Memory-Loaded Font"),
                    },
                };
                fonts_info.push(font_data)
            }
            Ok(fonts_info)
        }
        Err(e) => Err(format!("Error fetching font families: {}", e)),
    }
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
    let extension = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    match extension.as_deref() {
        Some("ttf") => "font/ttf",
        Some("otf") => "font/otf",
        Some("woff") => "application/font-woff",
        Some("woff2") => "application/font-woff2",
        Some("ttc") => "font/collection",
        _ => "application/octet-stream",
    }
}

#[tokio::main]
async fn main() {
    // 解决跨域
    let cors = warp::cors()
        .allow_any_origin() // 或者使用 .allow_origin("http://example.com") 指定具体的域
        .allow_headers(vec!["Content-Type"]) // 指定允许的头部
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]); // 指定允许的方法

    // 获取所有字体  eg:http://127.0.0.1:3030/fonts
    let fonts_route = warp::path("fonts").map(|| match get_system_fonts() {
        Ok(fonts) => {
            let json = json!({ "code": 200,"data": fonts });
            warp::reply::json(&json)
        }
        Err(error) => {
            let error_json = json!({ "error": error });
            warp::reply::json(&error_json)
        }
    });
    // 将字体转换成web地址 eg:http://127.0.0.1:3030/font?path=C:\\WINDOWS\\FONTS\\MSYHBD.TTC
    let convert_path = warp::path("font")
        .and(warp::query::<HashMap<String, String>>())
        .and_then(serve_font);

    // 组合所有路由
    let routes = fonts_route.or(convert_path).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
