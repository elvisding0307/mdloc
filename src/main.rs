//! # mdloc - Markdown图片本地化工具
//!
//! 这是一个用于处理Markdown文件中图片链接的工具，可以将远程图片下载并转换为：
//! - Base64内嵌格式（默认）
//! - 本地文件引用（使用--non-inline参数）
//!
//! ## 功能特性
//!
//! - 自动识别和下载Markdown中的图片链接
//! - 支持多种图片格式（PNG, JPEG, GIF, WebP等）
//! - 智能重试机制
//! - 模块化设计，易于维护和扩展
//!
//! ## 使用示例
//!
//! ```bash
//! # 将图片转换为base64内嵌格式
//! mdloc document.md
//!
//! # 下载图片到本地文件
//! mdloc --non-inline document.md
//!
//! # 指定输出文件名
//! mdloc -o output.md document.md
//! ```

mod error;
mod http_client;
mod image_processor;
mod markdown_processor;
mod mime_utils;
mod url_utils;

use clap::Parser;
use std::fs;
use std::path::PathBuf;

use error::{ErrorContext, MdlocError, Result};
use http_client::HttpClient;
use markdown_processor::{MarkdownProcessor, MarkdownProcessorConfig};

#[derive(Parser)]
#[command(name = "mdloc")]
#[command(about = "Download images from a Markdown file and update paths")]
struct Args {
    /// Path to the Markdown file
    markdown_file: PathBuf,

    /// Download images to local files instead of embedding as base64 data URIs
    #[arg(long)]
    non_inline: bool,

    /// Output file name (default: add "new_" prefix to original filename)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // 创建HTTP客户端
    let http_client = HttpClient::default()
        .with_context(|| "Failed to create HTTP client".to_string())?;

    // 准备配置
    let config = prepare_config(&args)?;

    // 创建Markdown处理器并处理文件
    let processor = MarkdownProcessor::new(config, http_client);
    let result = processor.process(&args.markdown_file).await?;

    // 显示结果
    display_results(&result, &args);

    Ok(())
}

/// 准备处理器配置
fn prepare_config(args: &Args) -> Result<MarkdownProcessorConfig> {
    let inline_mode = !args.non_inline;

    // 获取 Markdown 文件的目录和文件名
    let markdown_dir = args
        .markdown_file
        .parent()
        .ok_or_else(|| MdlocError::InvalidPath("Failed to get parent directory".to_string()))?;
    let markdown_name = args
        .markdown_file
        .file_name()
        .ok_or_else(|| MdlocError::InvalidPath("Failed to get file name".to_string()))?
        .to_string_lossy();

    // 确定输出文件名
    let output_file = if let Some(output) = &args.output {
        if output.is_absolute() {
            output.clone()
        } else {
            markdown_dir.join(output)
        }
    } else {
        markdown_dir.join(format!("new_{}", markdown_name))
    };

    // 只有在非inline模式下才创建本地文件夹
    let (images_folder, folder_base_name) = if !inline_mode {
        let folder_base_name = markdown_name.replace(".md", "") + "_images";
        let images_folder = markdown_dir.join(&folder_base_name);

        if !images_folder.exists() {
            fs::create_dir_all(&images_folder)
                .with_context(|| "Failed to create images directory".to_string())?;
        }

        (Some(images_folder), Some(folder_base_name))
    } else {
        (None, None)
    };

    Ok(MarkdownProcessorConfig {
        inline_mode,
        output_file,
        images_folder,
        folder_base_name,
    })
}

/// 显示处理结果
fn display_results(result: &markdown_processor::ProcessResult, args: &Args) {
    println!("\n✅ 处理完成！");

    let inline_mode = !args.non_inline;

    if inline_mode {
        println!(
            "📄 新的Markdown文件: {} (图片已内嵌为base64格式)",
            result.output_file.display()
        );
        println!("✅ 成功转换为base64格式 {} 张图片", result.successful_downloads);
    } else {
        if let Some(folder) = &result.images_folder {
            println!("📁 图片保存目录: {}", folder.display());
        }
        println!("📄 新的Markdown文件: {}", result.output_file.display());
        println!("✅ 成功下载 {} 张图片", result.successful_downloads);
    }

    if result.successful_downloads < result.total_images {
        println!(
            "⚠️  有 {} 张图片处理失败",
            result.total_images - result.successful_downloads
        );
    }
}
