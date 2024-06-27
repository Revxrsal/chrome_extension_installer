use std::io;

use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

pub fn install_extension(extension_id: &str) -> io::Result<()> {
    let key = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = key.create_subkey(format!("Software\\Google\\Chrome\\Extensions\\{}", extension_id))?;
    key.set_value("update_url", &"https://clients2.google.com/service/update2/crx")?;
    Ok(())
}