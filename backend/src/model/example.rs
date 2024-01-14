use rocket::serde::Serialize;

use super::base_model::BaseModel;

#[derive(Clone, Copy)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Example<'a> {
    pub id: &'a str
}

impl BaseModel for Example<'_>{
    fn id(self) -> String {
        self.id.to_string()
    }
    
    fn populate_data() -> Vec<Example<'static>> {
        let mut examples = Vec::new();
        examples.push(Example {id: "1"});
        examples.push(Example {id: "2"});

        examples
    }
}
