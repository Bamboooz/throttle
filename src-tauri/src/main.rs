#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod hardware;

use tauri::{App, Manager, Position, PhysicalPosition, Window};

fn set_window_position(window: &Window) -> Result<(), Box<dyn std::error::Error>> {
    let screen = window.current_monitor()?.unwrap();

    let screen_position = screen.position();
    let screen_width = screen.size().width as i32;
    let window_width = window.outer_size()?.width as i32;
    let offset = 5;

    let window_position = PhysicalPosition {
        x: screen_position.x + screen_width - window_width - offset,
        y: screen_position.y + offset,
    };

    window.set_position(Position::Physical(window_position)).unwrap();

    Ok(())
}

fn initialize(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();

    set_window_position(&window).unwrap();

    window.set_ignore_cursor_events(true).unwrap();

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(initialize)
        .invoke_handler(tauri::generate_handler![
            hardware::hw_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
