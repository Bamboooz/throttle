#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod hardware;
pub mod gui;

use rdev;
use eframe::{egui, NativeOptions};
use egui::{Pos2, ViewportBuilder};

use crate::gui::app::Application;

fn main() -> eframe::Result {
    // remember that before running, use unused-features analyze and cargo build and then cargo run --release for max optimizations
    let (screen_width, _) = rdev::display_size().unwrap();
    
    let app_size = [205.0, 188.0];

    let spawn_pos = Pos2::new(screen_width as f32 - app_size[0] - 10.0, 10.0); // 10px is the window offest so it doesn't touch the screen corners
    
    let viewport = ViewportBuilder::default()
        .with_inner_size(app_size)
        .with_resizable(false)
        .with_taskbar(false)
        .with_transparent(true) // change to true later
        .with_visible(false) // not working for some reason
        .with_position(spawn_pos)
        .with_decorations(false)
        .with_always_on_top();
    
    let options = NativeOptions {
        viewport,
        ..Default::default()
    };
    
    eframe::run_native(
        "throttle",
        options,
        Box::new(|_cc| {
            Ok(Box::<Application>::default())
        }),
    )
}
