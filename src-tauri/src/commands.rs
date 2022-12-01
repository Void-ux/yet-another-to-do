use std::collections::BTreeMap;

use surrealdb::sql::{Strand, Value, Object};
use tauri::AppHandle;

use crate::ctx::Ctx;

#[tauri::command]
pub async fn greet(name: &str) -> Result<String, surrealdb::Error> {
    Ok(format! {"hi {} from rust", name})
}

#[tauri::command]
pub async fn create_task(table: &str ,mut params: Object, app: AppHandle) -> Result<(), surrealdb::Error>{
    let mut ctx = Ctx::from_app(app);
    params.insert("tb".into(), Value::from(Strand::from(table))).unwrap();

    ctx.set_var(params)
    .execute("create type::table($tb) content $data").await?;
    Ok(())
}
