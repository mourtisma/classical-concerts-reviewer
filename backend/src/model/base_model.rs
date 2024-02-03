use validator::Validate;

pub trait BaseModel<'a>: Sized + Clone + Copy + Validate {
    fn id(self) -> &'a str;
    fn populate_data() -> Vec<Self>;
}