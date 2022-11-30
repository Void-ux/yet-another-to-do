use std::{env, sync::Arc};

mod commands;
mod ctx;
mod database;
use commands::*;
use database::Connection;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    let db = Connection::new().await?;
    let db = Arc::new(db);

    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![greet, create_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
