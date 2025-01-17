use tauri::{
    menu::{Menu, MenuItem}, tray::TrayIconBuilder, window, Emitter, Manager
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;
            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .show_menu_on_left_click(true)
                .build(app)?;
            #[cfg(desktop)]
            {
                // let main_window = app.get_window("main").unwrap();
                let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, shortcut, event| {
                        println!("{:?}", shortcut);
                        if shortcut == &ctrl_n_shortcut {
                            match event.state() {
                              ShortcutState::Pressed => {
                                println!("Ctrl-N Pressed!");
                              }
                              ShortcutState::Released => {
                                println!("Ctrl-N Released!");
                                if main_window.is_visible().unwrap(){
                                    main_window.hide().unwrap();
                                }
                                else{
                                    main_window.set_always_on_top(true).unwrap();
                                    main_window.set_focus().unwrap();
                                    main_window.current_monitor().unwrap();
                                    
                                    main_window.show().unwrap();
                                }
                              }
                            }
                        }
                    })
                    .build(),
                )?;

                app.global_shortcut().register(ctrl_n_shortcut)?;
            }
            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
              window.hide().unwrap();
              api.prevent_close();
            }
            _ => {}
          })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
