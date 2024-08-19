///! TODO USE [`tokio::io::Result`] to replace them. 
type Error = &'static str;
pub const ERROR_REQUEST_FAILED: Error = "Request Failed";
pub const ERROR_PAGE_CONTENT: Error = "Page Content Error";
pub const ERROR_PARSE: Error = "Parse Error";
pub const ERROR_WEBVPN: Error = "WebVPN Error";
pub const ERROR_ACCOUNT_LOGIN: Error = "Account Login Error";
