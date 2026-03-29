use serde::Serialize;
use utoipa::ToSchema;

use entity::user::Model;

#[derive(Serialize, ToSchema)]
pub struct User {
    pub name: String,
    pub email: String,
}

impl From<Model> for User {
    fn from(value: Model) -> Self {
        Self {
            name: value.username,
            email: value.email,
        }
    }
}
