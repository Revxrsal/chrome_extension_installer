# Chrome extension installer
This library makes it easy to [install external extensions](https://developer.chrome.com/docs/extensions/how-to/distribute/install-extensions) on Google Chrome and other Chromium-based browsers.

This was made initially to be used with [Tauri](https://tauri.app/), however it is versatile and can be used anywhere.

## Example
```rust
use chrome_extension_installer::install_extension;

pub fn main() {
    install_extension("bcjindcccaagfpapjjmafapmmgkkhgoa").unwrap();
}
```