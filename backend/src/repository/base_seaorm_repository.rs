use std::{marker::PhantomData, vec};

use sea_orm::{sea_query::Table, ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, DbBackend, DbErr, EntityTrait, IntoActiveModel, PaginatorTrait, QueryOrder, QueryTrait, SqlErr, TryIntoModel};
use uuid::Uuid;
use validator::Validate;
use crate::{dto::{list_options_dto::ListOptionsDto, order_dto::OrderDto}, model::{example_sea_orm, prelude::*, sea_orm_search_params}, transformer::sea_orm_transformer::SeaOrmTransformer};
use super::{error::{ORMError, RepositoryError, RepositoryErrorType}, repository_result::RepositoryResult};

pub struct BaseSeaOrmRepository<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, EntityOrderDto, Transformer, AM> {
    pub connection: &'a DatabaseConnection,
    pub _phantom_sea_orm: PhantomData<SeaOrmModel>,
    pub _phantom_get: PhantomData<GetModelDto>,
    pub _phantom_create: PhantomData<CreateModelDto>,
    pub _phantom_update: PhantomData<UpdateModelDto>,
    pub _phantom_order: PhantomData<EntityOrderDto>,
    pub _phantom_transformer: PhantomData<Transformer>,
    pub _phantom_active_model: PhantomData<AM>
}

impl<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, EntityOrderDto, Transformer, AM> 
    BaseSeaOrmRepository<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, EntityOrderDto, Transformer, AM> where 
        SeaOrmModel: EntityTrait,
        EntityOrderDto: OrderDto,
        Transformer: SeaOrmTransformer<'a, GetModelDto, CreateModelDto, UpdateModelDto, EntityOrderDto, SeaOrmModel, AM>,
        AM: ActiveModelBehavior + std::marker::Send, {

    pub async fn get_many(&mut self, options: ListOptionsDto<EntityOrderDto>) -> Result<RepositoryResult<GetModelDto>, RepositoryError<'a>> where <SeaOrmModel as sea_orm::EntityTrait>::Model: Sync {
        let sea_orm_search_params = Transformer::list_options_to_search_params::<Transformer>(options);

        let mut selector = SeaOrmModel::find();
        for ob in sea_orm_search_params.order_by.unwrap_or(vec![]) {
            let (ord, col) = ob.clone();
            selector = selector.clone().order_by(ord, col);
        }

        let get_many_result: Result<Vec<<SeaOrmModel as sea_orm::EntityTrait>::Model>, DbErr>;
        let mut total_count: u64 = 0;
        let mut num_pages: Option<u64> = None;

        if let Some(page_size) = sea_orm_search_params.page_size {
            let paginator = selector.into_model().paginate(self.connection, page_size);
            let num_items_and_pages = paginator.num_items_and_pages().await.unwrap();
            
            get_many_result = paginator.fetch_page(sea_orm_search_params.page_number.unwrap_or(1) - 1).await;
            total_count = num_items_and_pages.number_of_items;
            num_pages = Some(num_items_and_pages.number_of_pages);
        } else {
            get_many_result = selector.clone().all(self.connection).await;
            let total_count_result = selector.count(self.connection).await;
            total_count = total_count_result.unwrap();
        }
        
        
        
        if get_many_result.is_err() {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                orm_error: Some(ORMError {
                    sea_orm_db_error: get_many_result.err(),
                    sea_orm_transaction_error: None,
                })
            })
        } else {
            match get_many_result.ok() {
                Some(items_list) => Ok(RepositoryResult::<GetModelDto> {
                    items: items_list.iter().map(|it| Transformer::entity_to_get_dto(it.clone())).collect(),
                    total_count,
                    num_pages
                }),
                None => Ok(RepositoryResult::<GetModelDto> {
                    items: vec![],
                    total_count,
                    num_pages

                }),    
            }
        }
    }

    pub async fn get_one(&mut self, item_id: &'a str) -> Result<GetModelDto, RepositoryError<'a>> 
    where <<SeaOrmModel as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType: From<Uuid> {
        let get_one_result = SeaOrmModel::find_by_id(Uuid::parse_str(item_id).unwrap()).one(self.connection).await;
        
        if let Err(get_one_error) = get_one_result {
            match get_one_error {
                DbErr::RecordNotFound(_) => Err(RepositoryError { //TODO use the message arg for logs
                    error_type: RepositoryErrorType::NotFound,
                    message: Some("Record was not found"),
                    orm_error: Some(ORMError {
                        sea_orm_db_error: Some(get_one_error),
                        sea_orm_transaction_error: None,
                    })
                }),
                _ => Err(RepositoryError {
                    error_type: RepositoryErrorType::Unknown,
                    message: Some("An unknow error occurred"),
                    orm_error: Some(ORMError {
                        sea_orm_db_error: Some(get_one_error),
                        sea_orm_transaction_error: None,
                    })
                })
            }
        } else if let Ok(item) = get_one_result {
            match item {
                Some(it) => Ok(Transformer::entity_to_get_dto(it)),
                _ => Err(RepositoryError {
                    error_type: RepositoryErrorType::NotFound,
                    message: Some("Record was not found"),
                    orm_error: None
                })
            }
        } else {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                orm_error: None
            })
        }
        
    }

    pub async fn create(&mut self, data: CreateModelDto) -> Result<GetModelDto, RepositoryError<'a>> where <<AM as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<AM> {
        let active_model = Transformer::dto_to_create_active_model(data);
        let insert_result = active_model.insert(self.connection).await;
        
        match insert_result {
            Err(db_error) => Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                orm_error: Some(ORMError {
                    sea_orm_db_error: Some(db_error),
                    sea_orm_transaction_error: None,
                })
            }),
            Ok(new_item) => Ok(Transformer::active_model_to_dto(new_item))
        }
    }

    pub async fn update(&mut self, item_id: &'a str, data: UpdateModelDto) -> Result<GetModelDto, RepositoryError<'a>> where 
    <<AM as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<AM>, 
    <<SeaOrmModel as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType: From<uuid::Uuid> {
        self.get_one(item_id).await?;
        let active_model = Transformer::dto_to_update_active_model(data, item_id);
        let update_result = active_model.update(self.connection).await;
        
        if let Err(update_error) = update_result {
                match update_error {
                    DbErr::RecordNotFound(_) => Err(RepositoryError { // TODO use the message arg for logs
                        error_type: RepositoryErrorType::NotFound,
                        message: Some("Record was not found"),
                        orm_error: Some(ORMError {
                            sea_orm_db_error: Some(update_error),
                            sea_orm_transaction_error: None,
                        })
                    }),
                    _ => Err(RepositoryError {
                        error_type: RepositoryErrorType::Unknown,
                        message: Some("An unknow error occurred"),
                        orm_error: Some(ORMError {
                            sea_orm_db_error: Some(update_error),
                            sea_orm_transaction_error: None,
                        })
                    })
                }
        } else if let Ok(updated_example) = update_result {
            Ok(Transformer::active_model_to_dto(updated_example)) 
        } else {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                orm_error: None
            })
        }
    }

    pub async fn delete(&mut self, item_id: &'a str) -> Result<(), RepositoryError<'a>> where <<SeaOrmModel as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType: From<Uuid> {
        let delete_result = SeaOrmModel::delete_by_id(Uuid::parse_str(item_id).unwrap()).exec(self.connection).await;

        if let Ok(res) = delete_result {
            if res.rows_affected > 0 {
                Ok(())
            } else {
                Err(RepositoryError {
                    error_type: RepositoryErrorType::NotFound,
                        message: None,
                        orm_error: None
                })
            }
        } else {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                orm_error: None
            })
        }
        
    }      

}

#[cfg(test)]
mod tests {
    use std::collections::{btree_map, BTreeMap};

    use crate::{dto::{example_dto::{ExampleCreateDto, ExampleGetDto, ExampleOrderDto, ExampleUpdateDto}, list_options_dto::OrderType}, transformer::example_transformer::ExampleTransformer};

    use super::*;
    use chrono::Utc;
    use sea_orm::{
        entity::prelude::*, DatabaseBackend, ItemsAndPagesNumber, MockDatabase, MockExecResult, MockRow, Transaction
    };

    #[async_std::test]
    async fn test_get_many() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![BTreeMap::from([("num_items".to_string(), Value::BigInt(Some(1)))])]])
            .append_query_results([
                // First query result
                vec![
                    ExampleSeaOrmModel {
                        id: Uuid::new_v4(),
                        name: String::from("Example 1"),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc()
                    },
                    ExampleSeaOrmModel {
                        id: Uuid::new_v4(),
                        name: String::from("Example 2"),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc()
                    }
                ]
            ])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let examples_result = repository.get_many(ListOptionsDto::<ExampleOrderDto> {
                order_by: Some(vec![ExampleOrderDto { field: String::from("name"), direction: OrderType::Asc }]),
                page: Some(1),
                limit: Some(1)
            }).await;

            let examples = examples_result.ok().unwrap();

            assert_eq!(examples.total_count, 1);
            assert_eq!(examples.num_pages, Some(1));

        Ok(())
    }

    #[async_std::test]
    async fn test_get_many_error() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![BTreeMap::from([("num_items".to_string(), Value::BigInt(Some(1)))])]])
            .append_query_errors(vec![DbErr::RecordNotFound(String::from("error"))])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let examples_result = repository.get_many(ListOptionsDto::<ExampleOrderDto> {
                order_by: Some(vec![ExampleOrderDto { field: String::from("name"), direction: OrderType::Asc }]),
                page: Some(1),
                limit: Some(1)
            }).await;

            assert!(examples_result.is_err());

        Ok(())
    }

    #[async_std::test]
    async fn test_get_one() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // First query result
                vec![
                    ExampleSeaOrmModel {
                        id,
                        name: String::from("Example 1"),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc()
                    }
                ]
            ])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_result = repository.get_one(id_str).await;

            let example = example_result.ok().unwrap();

            assert_eq!(example.id, id.to_string());

        Ok(())
    }

    #[async_std::test]
    async fn test_get_one_not_found() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::RecordNotFound(String::from("Not found"))])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_result = repository.get_one(id_str).await;

            assert!(example_result.is_err());

            assert_eq!(example_result.err().unwrap().message, Some("Record was not found"));

        Ok(())
    }

    #[async_std::test]
    async fn test_get_one_error() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::RecordNotInserted])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_result = repository.get_one(id_str).await;

            assert!(example_result.is_err());

            assert_eq!(example_result.err().unwrap().message, Some("An unknow error occurred"));

        Ok(())
    }

    #[async_std::test]
    async fn test_create() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // First query result
                vec![
                    ExampleSeaOrmModel {
                        id: Uuid::new_v4(),
                        name: String::from("Example 1"),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc()
                    }
                ]
            ])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let example_create_dto = ExampleCreateDto {
                name: Some(String::from("Example 1"))
            };

            let example_create_result = repository.create(example_create_dto).await;

            let example = example_create_result.ok().unwrap();

            assert!(example.id.len() > 0);

        Ok(())
    }

    #[async_std::test]
    async fn test_create_error() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_errors(vec![DbErr::RecordNotInserted])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let example_create_dto = ExampleCreateDto {
                name: Some(String::from("Example 1"))
            };

            let example_create_result = repository.create(example_create_dto).await;

            assert!(example_create_result.is_err());

            assert_eq!(example_create_result.err().unwrap().message, Some("An unknow error occurred"));

        Ok(())
    }

    #[async_std::test]
    async fn test_update() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // First query result
                vec![
                    ExampleSeaOrmModel {
                        id: Uuid::new_v4(),
                        name: String::from("Example 1"),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc()
                    }
                ],
                vec![
                    ExampleSeaOrmModel {
                        id: Uuid::new_v4(),
                        name: String::from("Example 1"),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc()
                    }
                ]
            ])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let example_update_dto = ExampleUpdateDto {
                name: Some(String::from("Example 1"))
            };

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_update_result = repository.update(id_str, example_update_dto).await;

            let example = example_update_result.ok().unwrap();

            assert!(example.id.len() > 0);

        Ok(())
    }

    #[async_std::test]
    async fn test_update_not_found() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::RecordNotFound(String::from("Not found"))])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let example_update_dto = ExampleUpdateDto {
                name: Some(String::from("Example 1"))
            };

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_update_result = repository.update(id_str, example_update_dto).await;

            assert!(example_update_result.is_err());

            assert_eq!(example_update_result.err().unwrap().message, Some("Record was not found"));

        Ok(())
    }

    #[async_std::test]
    async fn test_update_get_error() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::RecordNotUpdated])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let example_update_dto = ExampleUpdateDto {
                name: Some(String::from("Example 1"))
            };

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_update_result = repository.update(id_str, example_update_dto).await;

            assert!(example_update_result.is_err());

            assert_eq!(example_update_result.err().unwrap().message, Some("An unknow error occurred"));

        Ok(())
    }

    #[async_std::test]
    async fn test_update_error() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            // First query result
            vec![
                ExampleSeaOrmModel {
                    id: Uuid::new_v4(),
                    name: String::from("Example 1"),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc()
                }
            ]
        ])
            .append_query_errors(vec![DbErr::RecordNotUpdated])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let example_update_dto = ExampleUpdateDto {
                name: Some(String::from("Example 1"))
            };

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_update_result = repository.update(id_str, example_update_dto).await;

            assert!(example_update_result.is_err());

            assert_eq!(example_update_result.err().unwrap().message, Some("An unknow error occurred"));

        Ok(())
    }

    #[async_std::test]
    async fn test_delete() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([
                MockExecResult {
                    rows_affected: 1,
                    last_insert_id: 0
                }
            ])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_delete_result = repository.delete(id_str).await;

            assert!(example_delete_result.is_ok());

        Ok(())
    }

    #[async_std::test]
    async fn test_delete_not_found() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        // Create MockDatabase with mock query results
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([
                MockExecResult {
                    rows_affected: 0,
                    last_insert_id: 0
                }
            ])
            .into_connection();

            let mut repository = BaseSeaOrmRepository {
                connection: &connection,
                _phantom_sea_orm: PhantomData::<ExampleSeaOrm>,
            _phantom_get: PhantomData::<ExampleGetDto>,
            _phantom_create: PhantomData::<ExampleCreateDto>,
            _phantom_update: PhantomData::<ExampleUpdateDto>,
            _phantom_order: PhantomData::<ExampleOrderDto>,
            _phantom_transformer: PhantomData::<ExampleTransformer>,
            _phantom_active_model: PhantomData::<ExampleActiveModel>
            };

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_delete_result = repository.delete(id_str).await;

            assert!(example_delete_result.is_err());

            assert_eq!(example_delete_result.err().unwrap().error_type, RepositoryErrorType::NotFound);

        Ok(())
    }
}