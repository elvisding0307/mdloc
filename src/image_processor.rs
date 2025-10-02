//! 图片处理模块
//!
//! 提供图片验证、格式检测和Base64编码功能。

use base64::{engine::general_purpose, Engine as _};
use sha1::{Digest, Sha1};


use crate::mime_utils::MimeType;
use crate::url_utils;

/// 图片处理器
/// 
/// 提供图片相关的各种处理功能，包括Base64转换、文件名生成等
pub struct ImageProcessor;

impl ImageProcessor {
    /// 将图片数据转换为base64 data URI格式
    pub fn to_base64_data_uri(
        image_data: &[u8],
        content_type: Option<&str>,
        url: Option<&str>,
    ) -> String {
        let mime_type = MimeType::determine(content_type, url);
        let base64_data = general_purpose::STANDARD.encode(image_data);
        format!("data:{};base64,{}", mime_type, base64_data)
    }

    /// 生成图片文件名（基于SHA-1哈希）
    pub fn generate_filename(
        image_data: &[u8],
        content_type: Option<&str>,
        url: &str,
    ) -> String {
        // 计算 SHA-1 值
        let mut hasher = Sha1::new();
        hasher.update(image_data);
        let sha1 = format!("{:x}", hasher.finalize());

        // 获取文件扩展名
        let extension = if let Some(ext) = url_utils::get_extension_from_url(url) {
            ext
        } else {
            // 根据 Content-Type 判断扩展名
            let ext = if let Some(ct) = content_type {
                MimeType::to_extension(ct)
            } else {
                "jpg"
            };

            println!(
                "Unknown image type for URL: {}, using .{} as default",
                url, ext
            );
            ext.to_string()
        };

        format!("{}.{}", sha1, extension)
    }

    /// 验证图片数据是否有效
    pub fn validate_image_data(data: &[u8]) -> bool {
        // 简单的图片格式验证
        if data.len() < 4 {
            return false;
        }

        // 检查常见图片格式的魔数
        match &data[0..4] {
            [0xFF, 0xD8, 0xFF, _] => true,        // JPEG
            [0x89, 0x50, 0x4E, 0x47] => true,     // PNG
            [0x47, 0x49, 0x46, 0x38] => true,     // GIF
            [0x42, 0x4D, _, _] => true,           // BMP
            _ => {
                // 检查WebP格式
                if data.len() >= 12 {
                    &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP"
                } else {
                    false
                }
            }
        }
    }
}