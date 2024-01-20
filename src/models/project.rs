use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset, Selectable, associations::Associations};
use crate::models::client::Client;


#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset, Selectable, Associations)]
#[diesel(belongs_to(Client))]
#[diesel(table_name = crate::repository::schema::projects)]
pub struct Project {
    #[serde(default)]
    pub id: String,
    pub client_id: String,
    pub name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
