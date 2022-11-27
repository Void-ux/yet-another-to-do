use crate::{error::*, prelude::*};
use std::collections::BTreeMap;
use std::{env, fs, path::Path};
use surrealdb::sql::{Datetime, Value};
use surrealdb::{Datastore, Session};

pub struct Connection {
    ds: Datastore,
    ses: Session,
}

impl Connection {
    pub async fn new() -> Result<Self> {
        let appdata = env::var("APPDATA")
            .expect("%APPDATA% is not set")
            .replace('\\', "/");

        let path = format!("{appdata}/yet-another-to-do/database");

        if !Path::new(&path).exists() {
            fs::create_dir(&path).expect("couldnt create dir");
        }

        let ds_string = "file://".to_string() + &path;

        let ds = Datastore::new(&ds_string).await?;
        let ses = Session::for_db("appns", "appdb");
        Ok(Self { ds, ses })
    }

    pub async fn exec_create<T>(&self, table: &str, data: T) -> Result<String>
    where
        T: Into<Value>,
    {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN id";

        let now = Datetime::default().timestamp_nanos();

        let vars = BTreeMap::from([
            ("ctime".into(), now.into()),
            ("tb".into(), table.into()),
            ("data".into(), data.into()),
        ]);

        let response = self.ds.execute(sql, &self.ses, Some(vars), false).await?;
        let first_val = response
            .into_iter()
            .next()
            .map(|r| r.result)
            .expect("id not returned")?;

        if let Value::Object(val) = first_val.first() {
            let res = val
                .get_key_value("id")
                .unwrap_or_else(|| panic!("property id not found on {}", table));
            Ok(res.0.to_owned())
        } else {
            Err(Error::StoreFailToCreate(format!(
                "exec_create {table}, nothing returned."
            )))
        }
    }
}
