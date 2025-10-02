use std::fmt;

/// 应用程序错误类型
#[derive(Debug)]
pub enum MdlocError {
    /// IO错误
    Io(std::io::Error),
    /// HTTP请求错误
    Http(reqwest::Error),
    /// URL解析错误
    UrlParse(url::ParseError),
    /// URL编码错误
    UrlEncoding(std::borrow::Cow<'static, str>),
    /// 正则表达式错误
    Regex(regex::Error),
    /// 图片验证错误
    InvalidImage(String),
    /// 文件路径错误
    InvalidPath(String),
    /// 通用错误
    Generic(String),
}

impl fmt::Display for MdlocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MdlocError::Io(err) => write!(f, "IO错误: {}", err),
            MdlocError::Http(err) => write!(f, "HTTP请求错误: {}", err),
            MdlocError::UrlParse(err) => write!(f, "URL解析错误: {}", err),
            MdlocError::UrlEncoding(err) => write!(f, "URL编码错误: {}", err),
            MdlocError::Regex(err) => write!(f, "正则表达式错误: {}", err),
            MdlocError::InvalidImage(msg) => write!(f, "无效图片: {}", msg),
            MdlocError::InvalidPath(msg) => write!(f, "无效路径: {}", msg),
            MdlocError::Generic(msg) => write!(f, "错误: {}", msg),
        }
    }
}

impl std::error::Error for MdlocError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MdlocError::Io(err) => Some(err),
            MdlocError::Http(err) => Some(err),
            MdlocError::UrlParse(err) => Some(err),
            MdlocError::Regex(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for MdlocError {
    fn from(err: std::io::Error) -> Self {
        MdlocError::Io(err)
    }
}

impl From<reqwest::Error> for MdlocError {
    fn from(err: reqwest::Error) -> Self {
        MdlocError::Http(err)
    }
}

impl From<url::ParseError> for MdlocError {
    fn from(err: url::ParseError) -> Self {
        MdlocError::UrlParse(err)
    }
}

impl From<regex::Error> for MdlocError {
    fn from(err: regex::Error) -> Self {
        MdlocError::Regex(err)
    }
}

impl From<anyhow::Error> for MdlocError {
    fn from(err: anyhow::Error) -> Self {
        MdlocError::Generic(err.to_string())
    }
}

/// 应用程序结果类型
pub type Result<T> = std::result::Result<T, MdlocError>;

/// 错误上下文扩展
pub trait ErrorContext<T> {
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ErrorContext<T> for std::result::Result<T, E>
where
    E: Into<MdlocError>,
{
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let base_error = e.into();
            let context = f();
            MdlocError::Generic(format!("{}: {}", context, base_error))
        })
    }
}