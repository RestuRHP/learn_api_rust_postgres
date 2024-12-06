use diesel::{AsChangeset, Insertable, Queryable};
use crate::schema::tb_user;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "tb_user"]
pub struct NewUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}

