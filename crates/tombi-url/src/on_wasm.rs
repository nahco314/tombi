pub fn url_from_file_path<P: AsRef<std::path::Path>>(_path: P) -> Result<url::Url, ()> {
    Err(())
}

pub fn url_to_file_path(_url: &url::Url) -> Result<std::path::PathBuf, ()> {
    Err(())
}
