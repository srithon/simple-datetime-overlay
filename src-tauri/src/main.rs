#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use gdk::DisplayManager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let display = DisplayManager::get()
                .default_display()
                .expect("There must be a default display.");

            let num_monitors = display.n_monitors();

            // now, let's go through each monitor and create a window on that monitor
            for i in 0..num_monitors {
                let monitor = display.monitor(i).expect(&format!(
                    "There must be a monitor associated with the number {}",
                    i
                ));

                let monitor_geometry = monitor.geometry();

                let _ = tauri::WindowBuilder::new(
                    app,
                    format!("{}", i),
                    tauri::WindowUrl::App("index.html".into()),
                )
                .position(monitor_geometry.x() as f64, monitor_geometry.y() as f64)
                .inner_size(
                    monitor_geometry.width() as f64,
                    monitor_geometry.height() as f64,
                )
                .decorations(false)
                .transparent(true)
                .fullscreen(true)
                .build()?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
