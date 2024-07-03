#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use tauri::{App, Manager, Window};
use tauri_plugin_positioner::{WindowExt, Position};
use global_hotkey::{GlobalHotKeyManager, GlobalHotKeyEvent, hotkey::{HotKey, Modifiers, Code}};

fn switch_visibility(window: Window) {
    if window.is_visible().unwrap() {
        window.hide().unwrap();
    } else {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

fn initialize_hotkey(window: Window) {
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::ALT), Code::KeyT);

    manager.register(hotkey).unwrap();
    let receiver = GlobalHotKeyEvent::receiver();

    std::thread::spawn(move || loop {
        if let Ok(_event) = receiver.try_recv() {
            switch_visibility(window.clone());
        }

        std::thread::sleep(Duration::from_millis(100));
    });
}

fn initialize(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();

    window.hide().unwrap();
    let _ = window.move_window(Position::TopRight);

    initialize_hotkey(window);

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(initialize)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
