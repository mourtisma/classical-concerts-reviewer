pub trait BaseModel: Sized {
    fn populate_data() -> Vec<Self>;
}