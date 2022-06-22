pub fn get_content(url: &str) -> String {
    let mut writer = Vec::new();
    http_req::request::get(url, &mut writer).unwrap();
    String::from_utf8_lossy(&writer).to_string()
}
