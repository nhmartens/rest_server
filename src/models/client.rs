use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};


#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::clients)]
pub struct Client {
    #[serde(default)]
    pub id: String,
    pub abbreviation: String,
    pub name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
