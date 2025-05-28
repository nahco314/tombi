use super::JsonCatalogSchema;
use tombi_url::url_from_file_path;

pub const DEFAULT_CATALOG_URL: &str = "https://www.schemastore.org/api/json/catalog.json";

#[derive(Debug, Clone, serde::Deserialize)]
pub struct JsonCatalog {
    pub schemas: Vec<JsonCatalogSchema>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatalogUrl(url::Url);

impl CatalogUrl {
    #[inline]
    pub fn new(url: url::Url) -> Self {
        Self(url)
    }
    
    #[inline]
    pub fn new_unchecked(url: String) -> Self {
        Self(url::Url::parse(&url).expect("Invalid URL"))
    }
    
    #[inline]
    pub fn from_file_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ()> {
        match url_from_file_path(path) {
            Ok(url) => Ok(Self(url)),
            Err(_) => Err(()),
        }
    }
}

impl std::ops::Deref for CatalogUrl {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for CatalogUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CatalogUrl> for url::Url {
    fn from(catalog_url: CatalogUrl) -> Self {
        catalog_url.0
    }
}
