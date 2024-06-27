use std::{fs, io};

use dirs::data_local_dir;

#[cfg(target_os = "macos")]
pub fn install_extension(extension_id: &str) -> io::Result<()> {
    let app_support = data_local_dir();
    if let Some(app_support) = app_support {
        let external_extensions_dir = app_support
            .join("Google")
            .join("Chrome")
            .join("External Extensions");
        let ext_file = external_extensions_dir.join(format!("{}.json", extension_id));
        fs::write(ext_file, r#"{"external_update_url": "https://clients2.google.com/service/update2/crx"}"#)?;
    }
    return Ok(());
}