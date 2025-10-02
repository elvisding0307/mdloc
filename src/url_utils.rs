//! URL处理工具模块
//!
//! 提供URL清理、验证和扩展名提取等功能。

use crate::error::Result;
use url::Url;

/// 常见图片后缀名
pub const COMMON_IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp"];

/// 清理和验证URL
pub fn clean_url(url: &str) -> Result<String> {
    let url = url.trim();

    // 处理URL编码
    let url = urlencoding::decode(url).unwrap_or_else(|_| url.into());

    // 确保URL以http://或https://开头
    let cleaned_url = if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else if url.starts_with("//") {
        format!("https:{}", url)
    } else if !url.starts_with('/') {
        format!("https://{}", url)
    } else {
        url.to_string()
    };

    Ok(cleaned_url)
}

/// 检查URL是否可能是图片URL
pub fn is_valid_image_url(url: &str) -> bool {
    if let Ok(parsed_url) = Url::parse(url) {
        let path = parsed_url.path().to_lowercase();

        // 检查路径是否包含图片扩展名
        for ext in COMMON_IMAGE_EXTENSIONS {
            if path.contains(&format!(".{}", ext)) {
                return true;
            }
        }
    }

    // 如果没有明显的扩展名，仍然尝试下载（可能是动态生成的图片）
    true
}

/// 从URL路径获取文件扩展名
pub fn get_extension_from_url(url: &str) -> Option<String> {
    if let Ok(parsed_url) = Url::parse(url) {
        let url_path = parsed_url.path();
        if let Some(pos) = url_path.rfind('.') {
            let extension = url_path[pos + 1..].to_lowercase();
            if COMMON_IMAGE_EXTENSIONS.contains(&extension.as_str()) {
                return Some(extension);
            }
        }
    }
    None
}