#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use enigo::{Enigo, MouseControllable};
use std::thread;
use tauri::{SystemTray, State};
use tauri::{
    ActivationPolicy, CustomMenuItem, GlobalShortcutManager, Manager, SystemTrayMenu,
    SystemTrayMenuItem,
};
use tauri_plugin_autostart::{MacosLauncher, AutoLaunchManager};

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
    x: i32,
    y: i32,
}

fn show_handler(app: tauri::AppHandle) {
    thread::spawn(move || {
        println!("Control+Shift+V was pressed");
        let enigo = Enigo::new();
        let cursor_location: (i32, i32) = enigo.mouse_location();
        app.emit_all(
            "shortcutPressed",
            Payload {
                message: "Control+Shift+V".into(),
                x: cursor_location.0,
                y: cursor_location.1,
            },
        )
        .unwrap();
    });
}

// fn hide_handler(app: tauri::AppHandle) {
//     thread::spawn(move || {
//         println!("Esc was pressed");
//         app.hide();
//     });
// }

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            tauri::SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                      std::process::exit(0);
                    },
                    _ => {}
                  }
            }
            _ => {}
        })
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .setup(|app| {
            app.hide();
            let mut manager = app.app_handle().global_shortcut_manager();
            let app_handle1 = app.app_handle();
            //let app_handle2 = app.app_handle();

            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            let launchManager: State<'_, AutoLaunchManager> = app.try_state().unwrap();
            launchManager.enable()?;

            manager
            .register("Control+Shift+V", move || show_handler(app_handle1.clone()))
                .ok()
                .unwrap();
            // manager
            // .register("Esc", move || hide_handler(app_handle2.clone()))
            //     .ok()
            //     .unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
