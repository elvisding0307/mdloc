//! MIME类型处理模块
//!
//! 提供MIME类型检测、转换和文件扩展名映射功能。

use crate::url_utils;

/// MIME类型映射工具
/// 
/// 提供各种MIME类型检测和转换方法
pub struct MimeType;

impl MimeType {
    /// 从Content-Type获取MIME类型
    pub fn from_content_type(content_type: &str) -> &'static str {
        let content_type = content_type.to_lowercase();

        if content_type.contains("image/jpeg") || content_type.contains("image/jpg") {
            "image/jpeg"
        } else if content_type.contains("image/png") {
            "image/png"
        } else if content_type.contains("image/gif") {
            "image/gif"
        } else if content_type.contains("image/bmp") {
            "image/bmp"
        } else if content_type.contains("image/tiff") {
            "image/tiff"
        } else if content_type.contains("image/webp") {
            "image/webp"
        } else if content_type.contains("image/svg") {
            "image/svg+xml"
        } else {
            "image/jpeg" // 默认为jpeg
        }
    }

    /// 从URL路径获取MIME类型
    pub fn from_url(url: &str) -> &'static str {
        if let Some(extension) = url_utils::get_extension_from_url(url) {
            Self::from_extension(&extension)
        } else {
            "image/jpeg"
        }
    }

    /// 从文件扩展名获取MIME类型
    pub fn from_extension(extension: &str) -> &'static str {
        match extension.to_lowercase().as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "bmp" => "image/bmp",
            "tiff" => "image/tiff",
            "webp" => "image/webp",
            "svg" => "image/svg+xml",
            _ => "image/jpeg",
        }
    }

    /// 从Content-Type获取文件扩展名
    pub fn to_extension(content_type: &str) -> &'static str {
        let content_type = content_type.to_lowercase();

        if content_type.contains("image/jpeg") || content_type.contains("image/jpg") {
            "jpg"
        } else if content_type.contains("image/png") {
            "png"
        } else if content_type.contains("image/gif") {
            "gif"
        } else if content_type.contains("image/bmp") {
            "bmp"
        } else if content_type.contains("image/tiff") {
            "tiff"
        } else if content_type.contains("image/webp") {
            "webp"
        } else {
            "jpg" // 默认使用jpg扩展名
        }
    }

    /// 智能确定MIME类型（优先使用Content-Type，回退到URL）
    pub fn determine(content_type: Option<&str>, url: Option<&str>) -> &'static str {
        if let Some(ct) = content_type {
            if ct.starts_with("image/") {
                Self::from_content_type(ct)
            } else if let Some(u) = url {
                Self::from_url(u)
            } else {
                "image/jpeg"
            }
        } else if let Some(u) = url {
            Self::from_url(u)
        } else {
            "image/jpeg"
        }
    }
}