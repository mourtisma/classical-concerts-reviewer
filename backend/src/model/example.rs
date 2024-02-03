use rocket::serde::{Serialize, Deserialize};
use validator::Validate;

use super::base_model::BaseModel;

#[derive(Clone, Copy, Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct Example<'a> {
    pub id: &'a str,

    #[validate(required)]
    pub name: Option<&'a str>
}

impl<'a> BaseModel<'a> for Example<'a>{
    fn id(self) -> &'a str {
        self.id
    }
    
    fn populate_data() -> Vec<Example<'a>> {
        let mut examples = Vec::new();
        examples.push(Example {id: "1", name: Some("Example 1")});
        examples.push(Example {id: "2", name: Some("Example 2")});

        examples
    }
}
