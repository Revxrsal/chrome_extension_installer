use std::{fs, io};
use std::path::Path;

#[cfg(target_os = "linux")]
pub fn install_extension(extension_id: &str) -> io::Result<()> {
    let external_extensions_dir = Path::new("/usr/share/google-chrome/extensions/");
    let ext_file = external_extensions_dir.join(format!("{}.json", extension_id));
    fs::write(ext_file, r#"{"external_update_url": "https://clients2.google.com/service/update2/crx"}"#)?;
    return Ok(());
}