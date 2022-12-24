#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern "C" {
    // source: https://github.com/gtk-rs/gdk/issues/182#issuecomment-321578087
    // TODO: why does this work?
    fn gdk_x11_window_get_xid(window: gdk::Window) -> u32;
}

use gdk::{prelude::ObjectExt, DisplayManager};
use std::process::Command;

use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(group(
            ArgGroup::new("monitor-spec")
                .required(false)
                .args(&["only_monitor", "active_monitor"]),
        ))]
struct Args {
    /// Which number monitor should be displayed
    #[arg(short, long)]
    only_monitor: Option<u8>,

    /// If set, only display on the monitor with the active, focused window
    #[arg(short, long, default_value_t = false)]
    active_monitor: bool,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn close(app_handle: tauri::AppHandle) -> () {
    app_handle.exit(0);
}

fn main() {
    let args = Args::parse();

    tauri::Builder::default()
        .setup(move |app| {
            let display = DisplayManager::get()
                .default_display()
                .expect("There must be a default display.");

            let get_geometry = |monitor_num| {
                let monitor = display.monitor(monitor_num).expect(&format!(
                    "There must be a monitor associated with the number {}",
                    monitor_num
                ));

                monitor.geometry()
            };

            let monitor_vector = if let Some(monitor_num) = args.only_monitor {
                vec![get_geometry(monitor_num as i32)]
            } else if args.active_monitor {
                let screen = display.default_screen();
                let mut monitor_option = None;

                // I couldn't figure out how to get the _NET_ACTIVE_WINDOW property
                // I tried to use root_window.property_value("_NET_ACTIVE_WINDOW") but it
                // didn't work.
                // same with display. and screen.
                // this python gist seems to use property_get to do what I wanted: https://gist.github.com/unhammer/1815146
                let xdotool_query = Command::new("xdotool").arg("getactivewindow").output()?;
                let output_string = String::from_utf8_lossy(&xdotool_query.stdout);
                let active_window_xid = output_string.trim().parse::<u32>()?;

                let windows = screen.window_stack();

                for window in windows {
                    let monitor_at_window = display.monitor_at_window(&window);
                    let xid = unsafe { gdk_x11_window_get_xid(window) };
                    if xid == active_window_xid {
                        // then, we have our window
                        // now, get the associated monitor
                        let _ = monitor_option.insert(
                            monitor_at_window
                                .expect("Must have a monitor associated with window")
                                .geometry(),
                        );
                        break;
                    }
                }

                if monitor_option.is_none() {
                    let _ = monitor_option.insert(get_geometry(0));
                }

                monitor_option.into_iter().collect()
            } else {
                // all monitors
                let num_monitors = display.n_monitors();
                (0..num_monitors).map(get_geometry).collect()
            };

            // now, let's go through each monitor and create a window on that monitor
            for (i, monitor_geometry) in monitor_vector.into_iter().enumerate() {
                let window = tauri::WindowBuilder::new(
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
                .always_on_top(true)
                .focused(false)
                .build()
                .expect("Window must be created successfully");

                window.set_ignore_cursor_events(true).unwrap();
                window.set_cursor_visible(false).unwrap();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![close])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
