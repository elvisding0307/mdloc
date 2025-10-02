//! HTTP客户端模块
//!
//! 提供图片下载、重试机制和HTTP请求处理功能。

use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

use crate::error::{ErrorContext, MdlocError, Result};

/// HTTP客户端配置
/// 
/// 包含超时时间、重试次数和重试延迟等配置项
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub timeout: Duration,
    pub max_retries: usize,
    pub retry_delay: Duration,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_secs(2),
        }
    }
}

/// HTTP客户端包装器
pub struct HttpClient {
    client: Client,
    config: HttpClientConfig,
}

impl HttpClient {
    /// 创建新的HTTP客户端
    pub fn new(config: HttpClientConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .with_context(|| "Failed to create HTTP client".to_string())?;

        Ok(Self { client, config })
    }

    /// 使用默认配置创建HTTP客户端
    pub fn default() -> Result<Self> {
        Self::new(HttpClientConfig::default())
    }

    /// 获取默认HTTP头部
    fn get_default_headers() -> HashMap<&'static str, &'static str> {
        let mut headers = HashMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36");
        headers.insert("Accept", "image/webp,image/apng,image/*,*/*;q=0.8");
        headers.insert("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8");
        headers.insert("Accept-Encoding", "gzip, deflate, br");
        headers.insert("Connection", "keep-alive");
        headers.insert("Upgrade-Insecure-Requests", "1");
        headers
    }

    /// 下载图片数据
    pub async fn download_image(&self, url: &str) -> Result<(Vec<u8>, Option<String>)> {
        let headers = Self::get_default_headers();

        for attempt in 0..self.config.max_retries {
            match self.try_download(url, &headers).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    println!("Error downloading {}: {}", url, e);
                    if attempt < self.config.max_retries - 1 {
                        println!(
                            "Retrying in {} seconds... (Attempt {}/{})",
                            self.config.retry_delay.as_secs(),
                            attempt + 2,
                            self.config.max_retries
                        );
                        sleep(self.config.retry_delay).await;
                    }
                }
            }
        }

        Err(MdlocError::Generic(format!(
            "Failed to download after {} attempts: {}",
            self.config.max_retries,
            url
        )))
    }

    /// 尝试下载（单次）
    async fn try_download(
        &self,
        url: &str,
        headers: &HashMap<&str, &str>,
    ) -> Result<(Vec<u8>, Option<String>)> {
        let mut request = self.client.get(url);
        for (key, value) in headers {
            request = request.header(*key, *value);
        }

        let response = request.send().await.with_context(|| "Failed to send request".to_string())?;

        if response.status().is_success() {
            let content_type = response
                .headers()
                .get("content-type")
                .and_then(|ct| ct.to_str().ok())
                .map(|s| s.to_lowercase());

            if let Some(ref ct) = content_type {
                if !ct.starts_with("image/") {
                    println!(
                        "Warning: Content-Type is not image for URL: {} (Content-Type: {})",
                        url, ct
                    );
                }
            }

            let bytes = response.bytes().await.with_context(|| "Failed to read response body".to_string())?;
            Ok((bytes.to_vec(), content_type))
        } else {
            Err(MdlocError::Generic(format!("HTTP {} for URL: {}", response.status(), url)))
        }
    }
}