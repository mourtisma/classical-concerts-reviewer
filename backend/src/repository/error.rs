pub enum RepositoryErrorType {
    NotFound,
    Unknown
}

pub struct RepositoryError {
    pub error_type: RepositoryErrorType,
    pub message: Option<String>

}