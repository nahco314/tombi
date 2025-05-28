mod document_link;
mod goto_definition;

pub use document_link::{document_link, DocumentLinkToolTip};
pub use goto_definition::goto_definition;
use tower_lsp::lsp_types::Url;

fn str2url(url: &str, tombi_toml_path: &std::path::Path) -> Option<Url> {
    if let Ok(target) = Url::parse(url) {
        Some(target)
    } else if let Some(tombi_config_dir) = tombi_toml_path.parent() {
        let mut file_path = std::path::PathBuf::from(url);
        if file_path.is_relative() {
            file_path = tombi_config_dir.join(file_path);
        }
        if file_path.exists() {
            Url::from_file_path(file_path).ok()
        } else {
            None
        }
    } else {
        None
    }
}
