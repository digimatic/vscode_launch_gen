use std::path::Path;

use serde_json::Value;

/// Trait for configuration providers
pub trait ConfigProvider {
    /// Returns the name of this configuration type
    fn name(&self) -> &'static str;

    /// Returns the configuration JSON for launch.json
    fn get_config(&self, params: Option<&str>) -> Value;

    /// Checks if this configuration type can be detected from a given file path
    fn can_detect_from_file(&self, path: &Path) -> bool;

    /// Checks if this configuration type can be detected from file content
    fn can_detect_from_content(&self, _filename: &str, _content: &str) -> bool {
        false // Default implementation returns false
    }
}
