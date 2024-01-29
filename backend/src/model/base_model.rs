pub trait BaseModel<'a>: Sized + Clone + Copy {
    fn id(self) -> &'a str;
    fn populate_data() -> Vec<Self>;
}