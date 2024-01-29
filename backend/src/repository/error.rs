pub enum RepositoryErrorType {
    NotFound,
    Unknown
}

pub struct RepositoryError<'a> {
    pub error_type: RepositoryErrorType,
    pub message: Option<&'a str>

}