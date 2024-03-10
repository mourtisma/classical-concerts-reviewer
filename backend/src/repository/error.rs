use sea_orm::{DbErr, TransactionError};

pub enum RepositoryErrorType {
    NotFound,
    Unknown
}

pub struct ORMError {
    pub sea_orm_db_error: Option<DbErr>,
    pub sea_orm_transaction_error: Option<TransactionError<DbErr>>
}

pub struct RepositoryError<'a> {
    pub error_type: RepositoryErrorType,
    pub message: Option<&'a str>,
    pub orm_error: Option<ORMError>

}