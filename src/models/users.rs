#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct UsersModel {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::NaiveDateTime>,
}
