use uuid::Uuid;
use validator::Validate;

pub trait BaseModel: Sized + Clone {
    fn id(self) -> String;
}