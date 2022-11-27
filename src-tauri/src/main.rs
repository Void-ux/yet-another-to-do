use std::{env, sync::Arc};
use tauri::{AppHandle, Manager};

mod database;
mod error;
mod prelude;

use database::Connection;
use prelude::*;


#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
async fn greet(name: &str, app: AppHandle) -> core::result::Result<String,()> {
    let conn = (*app.state::<Arc<Connection>>()).clone();
    let response = conn.exec_create("hi", "lmao").await;

    match response {
        Ok(_) => Ok(format!("Hello, {}! You've been greeted from Rust!", name)),
        Err(_) => {Err(())},
    }
}




#[tokio::main]
async fn main() -> Result<()> {
    let db = Connection::new().await.unwrap();
    let db = Arc::new(db);

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
