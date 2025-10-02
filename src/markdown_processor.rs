//! Markdown处理模块
//!
//! 负责识别、下载和转换Markdown文件中的图片链接。

use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{ErrorContext, Result};
use crate::http_client::HttpClient;
use crate::image_processor::ImageProcessor;
use crate::url_utils;

/// Markdown处理器配置
/// 
/// 包含输出模式（内联或文件）和输出路径等配置
#[derive(Debug, Clone)]
pub struct MarkdownProcessorConfig {
    pub inline_mode: bool,
    pub output_file: PathBuf,
    pub images_folder: Option<PathBuf>,
    pub folder_base_name: Option<String>,
}

/// Markdown处理器
pub struct MarkdownProcessor {
    config: MarkdownProcessorConfig,
    http_client: HttpClient,
}

impl MarkdownProcessor {
    /// 创建新的Markdown处理器
    pub fn new(config: MarkdownProcessorConfig, http_client: HttpClient) -> Self {
        Self {
            config,
            http_client,
        }
    }

    /// 处理Markdown文件
    pub async fn process(&self, markdown_file: &Path) -> Result<ProcessResult> {
        let mut content = fs::read_to_string(markdown_file)
            .with_context(|| "Failed to read markdown file".to_string())?;

        let image_urls = self.extract_image_urls(&content)?;
        println!("🔍 总共发现 {} 个图片链接", image_urls.len());

        let mut success_count = 0;

        for original_url in &image_urls {
            println!("Processing URL: {}", original_url);

            // 清理URL
            let cleaned_url = match url_utils::clean_url(original_url) {
                Ok(url) => {
                    println!("Cleaned URL: {}", url);
                    url
                }
                Err(e) => {
                    println!("Failed to clean URL {}: {}", original_url, e);
                    continue;
                }
            };

            // 检查是否是有效的图片URL
            if !url_utils::is_valid_image_url(&cleaned_url) {
                println!("Skipping non-image URL: {}", cleaned_url);
                continue;
            }

            // 下载并处理图片
            match self.http_client.download_image(&cleaned_url).await {
                Ok((image_data, content_type)) => {
                    // 验证图片数据
                    if !ImageProcessor::validate_image_data(&image_data) {
                        println!("Warning: Downloaded data may not be a valid image: {}", cleaned_url);
                    }

                    if self.config.inline_mode {
                        // inline模式：转换为base64 data URI
                        let data_uri = ImageProcessor::to_base64_data_uri(
                            &image_data,
                            content_type.as_deref(),
                            Some(&cleaned_url),
                        );
                        content = content.replace(original_url, &data_uri);
                        println!("Converted to base64: {} bytes", image_data.len());
                    } else {
                        // 普通模式：保存到本地文件
                        let image_name = ImageProcessor::generate_filename(
                            &image_data,
                            content_type.as_deref(),
                            &cleaned_url,
                        );
                        let image_path = self.config.images_folder.as_ref().unwrap().join(&image_name);

                        // 保存图片到本地
                        fs::write(&image_path, &image_data)
                            .with_context(|| "Failed to write image file".to_string())?;
                        println!(
                            "Downloaded: {} (Size: {} bytes)",
                            image_name,
                            image_data.len()
                        );

                        // 更新 Markdown 内容中的图片路径为相对路径
                        let relative_path = format!(
                            "{}/{}",
                            self.config.folder_base_name.as_ref().unwrap(),
                            image_name
                        );
                        content = content.replace(original_url, &relative_path);
                    }

                    success_count += 1;
                }
                Err(e) => {
                    println!("Failed to download: {}", e);
                }
            }
        }

        // 将更新后的内容写入新的 Markdown 文件
        fs::write(&self.config.output_file, &content)
            .with_context(|| "Failed to write output file".to_string())?;

        Ok(ProcessResult {
            total_images: image_urls.len(),
            successful_downloads: success_count,
            output_file: self.config.output_file.clone(),
            images_folder: self.config.images_folder.clone(),
        })
    }

    /// 提取Markdown中的图片URL
    fn extract_image_urls(&self, content: &str) -> Result<Vec<String>> {
        let re = Regex::new(r"!\[.*?\]\((.*?)\)").with_context(|| "Failed to compile regex".to_string())?;
        let image_urls: Vec<String> = re
            .captures_iter(content)
            .map(|cap| cap[1].to_string())
            .collect();
        Ok(image_urls)
    }
}

/// 处理结果
pub struct ProcessResult {
    pub total_images: usize,
    pub successful_downloads: usize,
    pub output_file: PathBuf,
    pub images_folder: Option<PathBuf>,
}