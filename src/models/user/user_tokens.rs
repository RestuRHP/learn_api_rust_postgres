use chrono::{DateTime, Utc};
use diesel::{Queryable, Insertable};
use uuid::Uuid;
use crate::schema::user_tokens;

#[derive(Queryable, Insertable)]
#[diesel(table_name = user_tokens)]
pub struct UserToken {
   pub id: Uuid,
   pub user_id: Uuid,
   pub token: String,
   pub created_at: Option<DateTime<Utc>>,
   pub expired_at: Option<DateTime<Utc>>,
}
