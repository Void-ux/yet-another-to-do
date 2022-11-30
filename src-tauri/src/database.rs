use std::collections::BTreeMap;
use std::{env, fs, path::Path};
use surrealdb::sql::Value;
use surrealdb::{Datastore, Session};

pub struct Connection {
    ds: Datastore,
    ses: Session,
}

impl Connection {
    pub async fn new() -> Result<Self, surrealdb::Error> {
        let appdata = env::var("APPDATA")
            .expect("%APPDATA% is not set")
            .replace('\\', "/");

        let path = format!("{appdata}/yetanothertodo");

        if !Path::new(&path).exists() {
            fs::create_dir(&path).expect("couldnt create dir");
        }

        let ds_string = "file://".to_string() + &path + "/database";

        let ds = Datastore::new(&ds_string).await?;
        let ses = Session::for_db("appns", "appdb");
        Ok(Self { ds, ses })
    }

    pub async fn execute(
        &self,
        sql: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<Vec<surrealdb::Response>, surrealdb::Error> {
        let parsed_sql = surrealdb::sql::parse(sql)?;
        self.ds
            .process(parsed_sql, &self.ses, vars.clone(), false)
            .await
    }
}
