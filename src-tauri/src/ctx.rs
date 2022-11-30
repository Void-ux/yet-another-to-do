use std::{collections::BTreeMap, sync::Arc};

use crate::database::Connection;
use surrealdb::sql::Value;
use tauri::{AppHandle, Manager};

pub struct Ctx {
    conn: Arc<Connection>,
    vars: Option<BTreeMap<String, Value>>,
}

impl Ctx {
    pub fn from_app(app_handle: AppHandle) -> Self {
        let conn = (*app_handle.state::<Arc<Connection>>()).clone();

        Ctx { conn, vars: None }
    }

    pub fn clear_vars(&mut self) -> &mut Self {
        self.vars = None;
        self
    }

    pub fn set_var(&mut self, var: surrealdb::sql::Object) -> &mut Self {
        if let Some(mut vars) = self.vars.take() {
            for (k, v) in var.iter() {
                vars.insert(k.to_owned(), v.to_owned());
            }
            self.vars = Some(vars);
            self
        } else {
            let mut new_map = BTreeMap::new();
            for (k, v) in var.iter() {
                new_map.insert(k.to_owned(), v.to_owned());
            }
            self.vars = Some(new_map);
            self
        }
    }

    pub async fn execute(&self, sql: &str) -> Result<Vec<surrealdb::Response>, surrealdb::Error> {
        self.conn.execute(sql, self.vars.clone()).await
    }
}
