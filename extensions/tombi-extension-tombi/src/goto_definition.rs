use tombi_config::TomlVersion;
use tombi_schema_store::dig_accessors;
use tower_lsp::lsp_types::TextDocumentIdentifier;

pub async fn goto_definition(
    text_document: &TextDocumentIdentifier,
    document_tree: &tombi_document_tree::DocumentTree,
    accessors: &[tombi_schema_store::Accessor],
    _toml_version: TomlVersion,
) -> Result<Option<Vec<tombi_extension::DefinitionLocation>>, tower_lsp::jsonrpc::Error> {
    // Check if current file is tombi.toml
    if !text_document.uri.path().ends_with("tombi.toml") {
        return Ok(Default::default());
    }

    let Some(tombi_toml_path) = text_document.uri.to_file_path().ok() else {
        return Ok(Default::default());
    };

    let mut locations = vec![];

    if accessors.last() == Some(&tombi_schema_store::Accessor::Key("path".to_string())) {
        if let Some((_, tombi_document_tree::Value::String(path))) =
            dig_accessors(document_tree, accessors)
        {
            if let Some(uri) = crate::str2url(path.value(), &tombi_toml_path) {
                locations.push(tombi_extension::DefinitionLocation {
                    uri,
                    range: tombi_text::Range::default(),
                });
            }
        }
    }

    if locations.is_empty() {
        return Ok(None);
    }

    Ok(Some(locations))
}
