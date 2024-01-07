pub trait BaseModel: Sized + Clone {
    fn populate_data() -> Vec<Self>;
}