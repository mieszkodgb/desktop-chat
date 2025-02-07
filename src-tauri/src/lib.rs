use tauri::{
    ipc::Channel, menu::{Menu, MenuItem}, tray::TrayIconBuilder, AppHandle, Emitter, Manager
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use ollama_rs::generation::completion::GenerationResponseStream;

mod ollama_api;
use tokio_stream::StreamExt;

#[tauri::command]
async fn ask_stream(app: AppHandle, input: String, sender: Channel<String>) {
    let mut stream: GenerationResponseStream = ollama_api::llama_stream(input).await;
    // sender.send("<answer>".to_string()).unwrap();
    let mut final_answer = false;
    while let Some(Ok(res)) = stream.next().await {
        for ele in res {
            // println!("Sending: {:?}", ele.response);
            let resp = ele.response.clone();
            if final_answer{
                sender.send(ele.response);
            }
            if resp == "</think>"{
                final_answer = true
            }
        }
    }
    // sender.send("</answer>".to_string()).unwrap();
}

#[tauri::command]
async fn ask(query: &str) -> Result<String, ()> {
    let response = ollama_api::llama_server(query.to_string()).await;
    let resp: Vec<&str> = response.split("</think>").collect();
    let thinking = resp[0].split("<think>").collect::<Vec<&str>>()[1];
    let answer = resp[1];
    println!("{:?}",answer);
    Ok(answer.to_string())
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![ask])
        .invoke_handler(tauri::generate_handler![ask_stream])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;
            let _tray: tauri::tray::TrayIcon = TrayIconBuilder::new()
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
                // let ctrl_i_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyI);
                // let esc_shortcut = Shortcut::new(Some(Modifiers::empty()), Code::Escape);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, shortcut, event| {
                        println!("Key pressed: {:?}", shortcut.key);
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
                                    main_window.current_monitor().unwrap();
                                    main_window.set_always_on_top(true).unwrap();
                                    main_window.show().unwrap();
                                    main_window.set_focus().unwrap();
                                    main_window.emit("focus-input", "test").unwrap();
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
        .on_window_event( |window, event| {
            if let tauri::WindowEvent::Focused(focused) = event {
                if !focused {
                    window.close().unwrap();
                }
            }
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
