pub const NETMOZI_URL: &str = "https://netmozi.com/";
pub const TITLE: &str = "<div class=\"recommended_block_name\">";
pub const TITLE_END: &str = "<br />";
pub const LANGUAGE: &str = "title=\"";
pub const LANGUAGE_END: &str = "\">";
pub const QUALITY_END: &str = "</div>";

pub mod errors {
    pub const ERROR_TITLE: &str = "Failed to find title!";
    pub const ERROR_LANGUAGE: &str = "Failed to find language!";
    pub const ERROR_LANGUAGE_END: &str = "Failed to find language end!";
    pub const ERROR_QUALITY_END: &str = "Failed to find quality end!";
}
