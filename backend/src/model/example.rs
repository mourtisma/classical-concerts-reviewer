use rocket::serde::Serialize;

use super::base_model::BaseModel;

#[derive(Clone, Copy)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Example {
    pub id: u64
}

impl BaseModel for Example {
    fn populate_data() -> Vec<Example> {
        let mut examples = Vec::new();
        examples.push(Example {id: 1});
        examples.push(Example {id: 2});

        examples
    }
}
