#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{App, Manager};
use tauri_plugin_positioner::{WindowExt, Position};
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, hotkey::{HotKey, Modifiers, Code}};

mod cpu;
mod gpu;
mod ram;

fn initialize(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();

    window.hide().unwrap();
    let _ = window.move_window(Position::TopRight);

    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::ALT), Code::KeyT);

    manager.register(hotkey);

    if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
        println!("{:?}", event);
    }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(initialize)
        .invoke_handler(tauri::generate_handler![
            cpu::get_cpu_usage,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
