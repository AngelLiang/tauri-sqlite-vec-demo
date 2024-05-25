// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod state;

use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

// 这是一个宏，用于将 greet 函数转换为 Tauri 可以通过其前端框架调用的命令。
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(app_handle: AppHandle, name: &str) -> String {
    // Should handle errors instead of unwrapping here
    let result = app_handle.db(|db| database::add_vector(db)).unwrap();
    format!("sqlite-vec: {}", result)
}

fn main() {
    tauri::Builder::default()
        // 将 AppState 的实例（其 db 字段使用默认值初始化）添加到 Tauri 应用的状态管理中。
        .manage(AppState { db: Default::default() })
        // 注册 greet 函数作为一个命令处理器，使其可以被前端代码调用。
        .invoke_handler(tauri::generate_handler![greet])
        // 设置回调函数，用于应用程序启动时的初始化。在这里，它初始化数据库连接并将其存储在应用程序状态中。
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = database::initialize_database(&handle).expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
