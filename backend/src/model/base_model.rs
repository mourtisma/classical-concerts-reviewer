pub trait BaseModel: Sized + Clone + Copy {
    fn id(self) -> String;
    fn populate_data() -> Vec<Self>;
}