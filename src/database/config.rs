use crate::database::models::*;
use crate::database::schema::*;
use crate::error::SendError;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

pub struct AppConfig<T>
where
    T: Serialize + DeserializeOwned + Clone + Send,
{
    pub name: &'static str,
    pub default_value: T,
}

pub fn config_open_registration() -> AppConfig<bool> {
    AppConfig {
        name: "OPEN_REGISTRATION",
        default_value: false,
    }
}

pub fn config_site_name() -> AppConfig<String> {
    AppConfig {
        name: "SITE_NAME",
        default_value: "No Site Name".to_string(),
    }
}

impl<T> AppConfig<T>
where
    T: Serialize + DeserializeOwned + Clone + Send,
{
    pub fn get(&self, conn: &PgConnection) -> Result<T, SendError> {
        get_config(conn, &self)
    }

    pub fn set(
        &self,
        conn: &PgConnection,
        new_value: T,
    ) -> Result<(), SendError> {
        set_config(conn, &self, new_value)
    }
}

pub fn get_config<T>(
    conn: &PgConnection,
    config_data: &AppConfig<T>,
) -> Result<T, SendError>
where
    T: Serialize + DeserializeOwned + Clone + Send,
{
    let result = match config::dsl::config
        .filter(config::config_name.eq(config_data.name))
        .first::<Config>(conn)
        .optional()
    {
        Ok(result) => result,
        Err(err) => return Err(err.to_string().into()),
    };

    match result {
        Some(row) => match serde_json::from_str(&row.config_value) {
            Ok(deserialized) => Ok(deserialized),
            Err(err) => Err(err.to_string().into()),
        },
        None => Ok(config_data.default_value.clone()),
    }
}

pub fn set_config<T>(
    conn: &PgConnection,
    option: &AppConfig<T>,
    new_value: T,
) -> Result<(), SendError>
where
    T: Serialize + DeserializeOwned + Clone + Send,
{
    let serialized = match serde_json::to_string(&new_value) {
        Ok(serialized) => serialized,
        Err(err) => {
            return Err(err.to_string().into());
        }
    };

    let new_config = NewConfig {
        config_name: option.name.to_string(),
        config_value: serialized.clone(),
    };

    match diesel::insert_into(config::table)
        .values(&new_config)
        .on_conflict(config::config_name)
        .do_update()
        .set(config::config_value.eq(serialized.clone()))
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string().into()),
    }
}
