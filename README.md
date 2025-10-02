# mdloc - Markdown 图片本地化工具

[![Rust](https://img.shields.io/badge/rust-1.86+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-GPL%203.0-blue.svg)](LICENSE)

一个高效的 Rust 命令行工具，用于处理 Markdown 文件中的图片链接。可以将远程图片下载并转换为 Base64 内嵌格式或本地文件引用，让你的 Markdown 文档完全自包含或本地化。

## ✨ 功能特性

- 🔍 **智能识别**: 自动识别 Markdown 文件中的所有图片链接
- 📥 **批量下载**: 并发下载多张图片，提高处理效率
- 🔄 **格式转换**: 支持两种输出模式：
  - Base64 内嵌格式（默认）- 生成完全自包含的 Markdown 文件
  - 本地文件引用 - 下载图片到本地并更新链接
- 🛡️ **智能重试**: 内置重试机制，处理网络不稳定情况
- 🎯 **多格式支持**: 支持 PNG、JPEG、GIF、WebP 等常见图片格式
- ⚡ **异步处理**: 基于 Tokio 的异步架构，性能优异
- 🧩 **模块化设计**: 清晰的代码结构，易于维护和扩展
- 🔒 **错误处理**: 完善的错误处理机制，提供详细的错误信息

## 🚀 快速开始

### 安装

确保你已经安装了 Rust（1.86+）：

```bash
# 克隆仓库
git clone <repository-url>
cd mdloc

# 编译安装
cargo build --release

# 或者直接运行
cargo run -- --help
```

### 基本用法

```bash
# 将图片转换为 Base64 内嵌格式（默认模式）
cargo run -- document.md

# 下载图片到本地文件并更新引用
cargo run -- --non-inline document.md

# 指定输出文件名
cargo run -- -o output.md document.md

# 组合使用
cargo run -- --non-inline -o localized.md document.md
```

## 📖 详细说明

### 命令行选项

```
Usage: mdloc [OPTIONS] <MARKDOWN_FILE>

Arguments:
  <MARKDOWN_FILE>  要处理的 Markdown 文件路径

Options:
      --non-inline       下载图片到本地文件而不是嵌入为 base64 数据 URI
  -o, --output <OUTPUT>  输出文件名（默认：在原文件名前添加 "new_" 前缀）
  -h, --help             显示帮助信息
```

### 处理模式

#### 1. Base64 内嵌模式（默认）

将远程图片下载并转换为 Base64 格式，直接嵌入到 Markdown 文件中：

```bash
cargo run -- document.md
```

**优点**：
- 生成完全自包含的文件
- 无需额外的图片文件
- 便于分享和传输

**缺点**：
- 文件体积较大
- 不利于图片复用

#### 2. 本地文件模式

下载图片到本地 `[filename]_images/` 文件夹，并更新 Markdown 中的引用：

```bash
cargo run -- --non-inline document.md
```

**优点**：
- 保持文件结构清晰
- 图片可以复用
- 便于后续编辑

**缺点**：
- 需要维护图片文件夹
- 分享时需要包含图片文件

### 示例

项目包含了一个示例文件 `examples/test.md`，你可以直接测试：

```bash
# 测试 Base64 内嵌模式
cargo run -- examples/test.md

# 测试本地文件模式
cargo run -- --non-inline examples/test.md
```

#### 输入文件 (examples/test.md)：

```markdown
# 测试文档

这是一个测试文档，包含一些图片链接。

![测试图片1](https://httpbin.org/image/png)
![测试图片2](https://httpbin.org/image/jpeg)
```

#### Base64 模式输出：

生成 `examples/new_test.md`，图片被转换为 Base64 格式：

```markdown
![测试图片1](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAA...)
![测试图片2](data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD...)
```

#### 本地文件模式输出：

生成 `examples/new_test.md` 和 `examples/test_images/` 文件夹：

```markdown
![测试图片1](test_images/379f5137831350c900e757b39e525b9db1426d53.png)
![测试图片2](test_images/eb1db8fa7b8277f2de5d7b40d6cdbc708aac4e52.jpg)
```

## 🏗️ 项目结构

```
src/
├── main.rs              # 主程序入口和命令行处理
├── error.rs             # 统一错误处理
├── http_client.rs       # HTTP 客户端和下载逻辑
├── image_processor.rs   # 图片处理和 Base64 转换
├── markdown_processor.rs # Markdown 文件解析和处理
├── mime_utils.rs        # MIME 类型工具
└── url_utils.rs         # URL 处理工具
```

### 核心模块

- **HTTP Client**: 负责图片下载，包含重试机制和错误处理
- **Image Processor**: 处理图片验证、格式检测和 Base64 编码
- **Markdown Processor**: 解析 Markdown 文件，识别图片链接并进行替换
- **URL Utils**: 提供 URL 清理和验证功能
- **Error Handling**: 统一的错误类型和处理机制

## ⚠️ 注意事项

### 网络要求
- 需要稳定的网络连接来下载远程图片
- 某些网站可能有防爬虫机制，可能导致下载失败
- 大文件下载可能需要较长时间

### 文件大小
- **Base64 模式**: 会显著增加文件大小（约增加 33%）
- **本地文件模式**: 推荐用于包含大量图片的文档

### 安全性
- 工具会下载并处理远程内容，请确保图片来源可信
- 建议在处理前检查 Markdown 文件中的图片链接

## 🔧 配置

程序使用内置的默认配置，包括：

- **超时时间**: 30 秒
- **最大重试次数**: 3 次
- **重试延迟**: 1 秒
- **用户代理**: 模拟现代浏览器

未来版本将支持配置文件自定义这些参数。

## 📊 性能和限制

### 性能特点
- **并发下载**: 支持多个图片同时下载
- **智能重试**: 失败时自动重试最多 3 次
- **内存优化**: 流式处理，避免大文件占用过多内存
- **缓存机制**: 相同 URL 不会重复下载

### 已知限制
- **文件大小**: Base64 模式会增加约 33% 的文件大小
- **网络依赖**: 需要稳定的网络连接
- **格式支持**: 仅支持常见图片格式
- **单线程**: 当前版本使用单线程处理（未来可能优化）

## 🔧 故障排除

### 常见问题

**Q: 图片下载失败怎么办？**
A: 
- 检查网络连接
- 确认图片 URL 是否有效
- 某些网站可能需要特定的 User-Agent 或 Headers

**Q: 生成的文件很大怎么办？**
A: 
- 使用 `--non-inline` 模式生成本地文件引用
- 考虑压缩原始图片

**Q: 程序运行很慢怎么办？**
A: 
- 大量图片下载需要时间，这是正常现象
- 网络速度会影响处理时间
- 可以考虑分批处理大文件

**Q: 某些图片格式不支持？**
A: 
- 目前支持常见的图片格式：PNG, JPEG, GIF, WebP, SVG, BMP, ICO
- 不支持的格式会保持原始链接

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 开发环境设置

```bash
# 克隆仓库
git clone <repository-url>
cd mdloc

# 安装依赖并构建
cargo build

# 运行
cargo run

# 运行示例
cargo run -- examples/test.md
```

## 📝 许可证

本项目采用 GPL 3.0 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🐛 问题反馈

如果你遇到任何问题或有功能建议，请在 [Issues](../../issues) 页面提交。

## 🙏 致谢

- [Tokio](https://tokio.rs/) - 异步运行时
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP 客户端
- [clap](https://github.com/clap-rs/clap) - 命令行参数解析
- [regex](https://github.com/rust-lang/regex) - 正则表达式支持

---

**Made with ❤️ in Rust**