#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use winreg::enums::*;
use winreg::RegKey;

fn main() {
    // Windows wallpaper path registry key:
    // HKEY_CURRENT_USER\Control Panel\Desktop\Wallpaper
    let wallpaper_path: String = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("Control Panel\\Desktop").expect("Failed to open registry key")
        .get_value("Wallpaper").expect("Failed to read registry value");

    showfile::show_path_in_file_manager(wallpaper_path);
}
