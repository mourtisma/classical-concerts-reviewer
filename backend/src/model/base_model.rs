use uuid::Uuid;
use validator::Validate;

pub trait BaseModel: Sized + Clone + Validate {
    fn id(self) -> String;
}