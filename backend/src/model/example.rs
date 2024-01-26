use rocket::serde::{Serialize, Deserialize};

use super::base_model::BaseModel;

#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Example<'a> {
    pub id: &'a str
}

impl<'a> BaseModel for Example<'a>{
    fn id(self) -> String {
        self.id.to_string()
    }
    
    fn populate_data() -> Vec<Example<'a>> {
        let mut examples = Vec::new();
        examples.push(Example {id: "1"});
        examples.push(Example {id: "2"});

        examples
    }
}
