use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct UserSerializer {
    pub id: Option<Uuid>,
    #[validate(email)]
    pub email: String,
    pub password: String,
}
