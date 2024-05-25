/*
这段代码的主要作用是在 tauri 应用中提供一种安全地访问和操作SQLite数据库的方式。通过 AppState 和 ServiceAccess trait，应用可以在多线程环境下安全地访问数据库连接，并执行数据库操作。

ref: https://42share.com/gpt/wpS3ctAW5P
*/
// 导入 rusqlite crate（Rust的包）中的 Connection 类型，它用于与SQLite数据库建立和管理连接。
use rusqlite::Connection;
use tauri::{AppHandle, State, Manager};

// 定义结构体
pub struct AppState {
  pub db: std::sync::Mutex<Option<Connection>>,
}

// 定义接口
pub trait ServiceAccess {
  // 接收一个泛型闭包 F 作为参数，并返回一个泛型类型 TResult。
  fn db<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Connection) -> TResult;

  // 接收一个泛型闭包 F 作为参数，并返回一个泛型类型 TResult。
  fn db_mut<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut Connection) -> TResult;
}

// 为 tauri 的 AppHandle 类型实现了 ServiceAccess trait。
impl ServiceAccess for AppHandle {

  // db 方法: 获取数据库连接（只读），锁定互斥锁以安全访问连接，并执行传入的操作（闭包 F）。
  fn db<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Connection) -> TResult {
    // self.state(): 这个方法从 AppHandle 获取应用的状态，这里的状态是之前定义的 AppState。
    let app_state: State<AppState> = self.state();
    let db_connection_guard = app_state.db.lock().unwrap();
    let db = db_connection_guard.as_ref().unwrap();
  
    operation(db)
  }

  // db_mut 方法: 类似于 db 方法，但它允许对数据库连接进行可变访问（可以修改数据）。
  fn db_mut<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut Connection) -> TResult {
    let app_state: State<AppState> = self.state();
    let mut db_connection_guard = app_state.db.lock().unwrap();
    let db = db_connection_guard.as_mut().unwrap();
  
    operation(db)
  }
}
