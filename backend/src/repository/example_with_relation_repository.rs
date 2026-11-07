use std::{marker::PhantomData, vec};

use sea_orm::{sea_query::{expr, BinOper, Expr, SimpleExpr, Table}, ActiveModelBehavior, ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, DbErr, DeleteResult, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect, RuntimeErr, SelectColumns, Set, SqlErr, TransactionError, TransactionTrait, TryIntoModel};
use uuid::Uuid;
use crate::{dto::{example_dto::ExampleOrderDto, example_with_relation_dto::{ExampleWithRelationCreateDto, ExampleWithRelationGetDto, ExampleWithRelationUpdateDto}, list_options_dto::ListOptionsDto}, model::{example_many_to_many, example_sea_orm_with_relation_example_many_to_many, prelude::*}, transformer::{example_with_relation_transformer::ExampleWithRelationTransformer, sea_orm_transformer::SeaOrmTransformer}};
use super::{error::{ORMError, RepositoryError, RepositoryErrorType}};

pub struct ExampleWithRelationRepository<'a> {
    pub connection: &'a DatabaseConnection
}

impl<'a> ExampleWithRelationRepository<'a> {

    pub async fn get_many(&mut self) -> Result<Vec<ExampleWithRelationGetDto>, RepositoryError<'a>> {
        
        let get_many_result = ExampleSeaOrmWithRelation::find().find_with_related(ExampleManyToMany).all(self.connection).await;
        
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
                Some(items_list) => Ok(items_list.iter().map(|it| ExampleWithRelationTransformer::entity_to_get_dto(it.clone())).collect()),
                None => Ok(vec![]),    
            }
        }
    }

    pub async fn get_one(&mut self, item_id: &'a str) -> Result<ExampleWithRelationGetDto, RepositoryError<'a>> {
        let get_one_result = ExampleSeaOrmWithRelation::find_by_id(Uuid::parse_str(item_id).unwrap()).find_with_related(ExampleManyToMany).all(self.connection).await;
        
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
        } else if let Ok(items) = get_one_result {
            match items.len() {
                1 => Ok(ExampleWithRelationTransformer::entity_to_get_dto(items[0].clone())),
                0 => Err(RepositoryError {
                    error_type: RepositoryErrorType::NotFound,
                    message: Some("Record was not found"),
                    orm_error: None
                }),
                _ => Err(RepositoryError {
                    error_type: RepositoryErrorType::Unknown,
                    message: Some("An unknow error occurred"),
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

    pub async fn create(&mut self, data: ExampleWithRelationCreateDto) -> Result<ExampleWithRelationGetDto, RepositoryError<'a>> {
        let (example_with_relation_active_model, example_many_to_many_active_models) = ExampleWithRelationTransformer::dto_to_create_active_model(data);
        let insert_result = self.connection.transaction(|txn| {
            Box::pin(async move {
                let mut associative_records: Vec<ExampleSeaOrmWithRelationExampleManyToManyActiveModel> = vec![];
                let example_with_relation_entity_id = ExampleSeaOrmWithRelation::insert(example_with_relation_active_model).exec(txn).await?.last_insert_id;
                for example_many_to_many_active_model in example_many_to_many_active_models {
                    let example_many_to_many_id = ExampleManyToMany::insert(example_many_to_many_active_model).exec(txn).await?.last_insert_id;
                    associative_records.push(ExampleSeaOrmWithRelationExampleManyToManyActiveModel {
                        example_sea_orm_with_relation_id: Set(example_with_relation_entity_id),
                        example_many_to_many_id: Set(example_many_to_many_id)
                    });
                }
                ExampleSeaOrmWithRelationExampleManyToMany::insert_many(associative_records).exec(txn).await?;
                
                let new_entity = &ExampleSeaOrmWithRelation::find_by_id(example_with_relation_entity_id).find_with_related(ExampleManyToMany).all(txn).await.unwrap()[0];
                Ok(new_entity.clone())

                
            })
        }).await;

        if let Err(txn_error) = insert_result {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                orm_error: Some(ORMError {
                    sea_orm_db_error: None,
                    sea_orm_transaction_error: Some(txn_error),
                })
            })
        } else if let Ok(new_entity) = insert_result {
            Ok(ExampleWithRelationTransformer::entity_to_get_dto(new_entity.clone()))
        } else {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                orm_error: None
            })
        }
    }

    pub async fn update(&mut self, item_id: &'a str, data: ExampleWithRelationUpdateDto) -> Result<ExampleWithRelationGetDto, RepositoryError<'a>> {
        self.get_one(item_id).await?;
        let (example_with_relation_active_model, example_many_to_many_active_models) = ExampleWithRelationTransformer::dto_to_update_active_model(data, item_id);
        let update_result = self.connection.transaction(|txn| {
            Box::pin(async move {
                let mut associative_records: Vec<ExampleSeaOrmWithRelationExampleManyToManyActiveModel> = vec![];
                let example_with_relation_entity_id = example_with_relation_active_model.update(txn).await?.id;
                
                let examples_many_to_many_to_create: Vec<&ExampleManyToManyActiveModel> = example_many_to_many_active_models.iter().filter(|e| e.id.is_not_set()).collect();
                let examples_many_to_many_to_update: Vec<&ExampleManyToManyActiveModel> = example_many_to_many_active_models.iter().filter(|e| e.id.is_set()).collect();
                
                // Create the related entities that need to be created
                let mut examples_many_to_many_ids = vec![];
                for example_many_to_many_active_model in examples_many_to_many_to_create {
                    let example_many_to_many_id = ExampleManyToMany::insert(example_many_to_many_active_model.clone()).exec(txn).await?.last_insert_id;
                    associative_records.push(ExampleSeaOrmWithRelationExampleManyToManyActiveModel {
                        example_sea_orm_with_relation_id: Set(example_with_relation_entity_id),
                        example_many_to_many_id: Set(example_many_to_many_id)
                    });

                    examples_many_to_many_ids.push(example_many_to_many_id);
                }
                ExampleSeaOrmWithRelationExampleManyToMany::insert_many(associative_records).exec(txn).await?;
                
                // Update the related entities that need to be updated
                for example_many_to_many_active_model in &examples_many_to_many_to_update {
                    example_many_to_many_active_model.clone().clone().update(txn).await?;
                }
                
                // Gather all the IDs and delete the associations of the related entities which are not in the payload
                let mut examples_many_to_many_to_update_ids = examples_many_to_many_to_update.iter().map(|e| e.id.clone().into_value().unwrap().as_ref_uuid().unwrap().to_owned()).collect();
                examples_many_to_many_ids.append(&mut examples_many_to_many_to_update_ids);
                let removed_examples_many_to_many = ExampleSeaOrmWithRelationExampleManyToMany::find().filter(Expr::col(example_sea_orm_with_relation_example_many_to_many::Column::ExampleManyToManyId).is_not_in(examples_many_to_many_ids)
                .and(Expr::col(example_sea_orm_with_relation_example_many_to_many::Column::ExampleSeaOrmWithRelationId).eq(example_with_relation_entity_id))).all(txn).await?;
                
                let removed_examples_many_to_many_ids: Vec<Uuid> = removed_examples_many_to_many.iter().map(|e| e.example_many_to_many_id).collect();
                ExampleSeaOrmWithRelationExampleManyToMany::delete_many().filter(Expr::col(example_sea_orm_with_relation_example_many_to_many::Column::ExampleManyToManyId).is_in(removed_examples_many_to_many_ids.clone())).exec(txn).await?;
                
                // Delete the related entities which need to be related
                ExampleManyToMany::delete_many().filter(Expr::col(example_many_to_many::Column::Id).is_in(removed_examples_many_to_many_ids)).exec(txn).await?;    


                let new_entity = &ExampleSeaOrmWithRelation::find_by_id(example_with_relation_entity_id).find_with_related(ExampleManyToMany).all(txn).await.unwrap()[0];
                Ok(new_entity.clone())

                
            })
        }).await;
        
        if let Err(txn_error) = update_result {
            Err(RepositoryError {
                        error_type: RepositoryErrorType::Unknown,
                        message: Some("An unknow error occurred"),
                        orm_error: Some(ORMError {
                            sea_orm_db_error: None,
                            sea_orm_transaction_error: Some(txn_error),
                        })
                    })
        } else if let Ok(new_entity) = update_result {
            Ok(ExampleWithRelationTransformer::entity_to_get_dto(new_entity.clone()))
        } else {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                orm_error: None
            })
        }
    } 

   pub async fn delete(&mut self, item_id: &'a str) -> Result<(), RepositoryError<'a>> {
    let owned_item_id = item_id.to_owned();
    let txn_delete_result = self.connection.transaction::<_, Option<u64>, DbErr>(|txn| {
        Box::pin(async move {
            let removed_examples_many_to_many = ExampleSeaOrmWithRelationExampleManyToMany::find().filter(Expr::col(example_sea_orm_with_relation_example_many_to_many::Column::ExampleSeaOrmWithRelationId).eq(Uuid::parse_str(&owned_item_id).unwrap())).all(txn).await?;
            
            let removed_examples_many_to_many_ids: Vec<Uuid> = removed_examples_many_to_many.iter().map(|e| e.example_many_to_many_id).collect();
            ExampleSeaOrmWithRelationExampleManyToMany::delete_many().filter(Expr::col(example_sea_orm_with_relation_example_many_to_many::Column::ExampleManyToManyId).is_in(removed_examples_many_to_many_ids.clone())).exec(txn).await?;
            
            // Delete the related entities which need to be related
            ExampleManyToMany::delete_many().filter(Expr::col(example_many_to_many::Column::Id).is_in(removed_examples_many_to_many_ids)).exec(txn).await?;    

            let delete_result = ExampleSeaOrmWithRelation::delete_by_id(Uuid::parse_str(&owned_item_id).unwrap()).exec(txn).await;

            if let Ok(res) = delete_result {
                Ok(Some(res.rows_affected))
            } else {
                Ok(None)
            }
    })
    }).await;
        
        if let Ok(rows_affected) = txn_delete_result {
            if let Some(ra) = rows_affected {
                if ra > 0 { Ok(()) } else {Err(RepositoryError {
                    error_type: RepositoryErrorType::NotFound,
                        message: None,
                        orm_error: None
                })}
            } else {
                Err(RepositoryError {
                    error_type: RepositoryErrorType::Unknown,
                    message: Some("An unknow error occurred"),
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

    use crate::{dto::{example_dto::{ExampleCreateDto, ExampleGetDto, ExampleOrderDto, ExampleUpdateDto}, example_many_to_many_dto::ExampleManyToManyCreateDto, list_options_dto::OrderType}, transformer::example_transformer::ExampleTransformer};

    use super::*;
    use chrono::Utc;
    use sea_orm::{
        entity::prelude::*, DatabaseBackend, ItemsAndPagesNumber, MockDatabase, MockExecResult, MockRow, Transaction
    };

    fn setup_repository<'a>(connection: &'a DatabaseConnection) -> 
    ExampleWithRelationRepository<'a> {
        ExampleWithRelationRepository {
            connection,
        }
    }

    #[async_std::test]
    async fn test_get_many() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            vec![
                (
                    ExampleSeaOrmWithRelationModel {
                        id,
                        example_id: Uuid::new_v4(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    },
                    Some(
                        ExampleManyToManyModel {
                            id: Uuid::new_v4(),
                            name: String::from("Example 1"),
                            created_at: Utc::now().naive_utc(),
                            updated_at: Utc::now().naive_utc(),
                        }
                    )
                ),
                (
                    ExampleSeaOrmWithRelationModel {
                        id,
                        example_id: Uuid::new_v4(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    },
                    Some(
                        ExampleManyToManyModel {
                            id: Uuid::new_v4(),
                            name: String::from("Example 2"),
                            created_at: Utc::now().naive_utc(),
                            updated_at: Utc::now().naive_utc(),
                        }
                    )
                ),
            ]
        ])
        .into_connection();

            let mut repository = setup_repository(&connection);

            let result = repository.get_many().await;

            let items = result.ok().unwrap();

            assert_eq!(items.len(), 1);

        Ok(())
    }

    #[async_std::test]
    async fn test_get_many_error() -> Result<(), DbErr> {
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![BTreeMap::from([("num_items".to_string(), Value::BigInt(Some(1)))])]])
            .append_query_errors(vec![DbErr::RecordNotFound(String::from("error"))])
            .into_connection();

            let mut repository = setup_repository(&connection);

            let examples_result = repository.get_many().await;

            assert!(examples_result.is_err());

        Ok(())
    }

    #[async_std::test]
    async fn test_get_one() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            vec![
                (
                    ExampleSeaOrmWithRelationModel {
                        id,
                        example_id: Uuid::new_v4(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    },
                    Some(
                        ExampleManyToManyModel {
                            id: Uuid::new_v4(),
                            name: String::from("Example 1"),
                            created_at: Utc::now().naive_utc(),
                            updated_at: Utc::now().naive_utc(),
                        }
                    )
                ),
                (
                    ExampleSeaOrmWithRelationModel {
                        id,
                        example_id: Uuid::new_v4(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    },
                    Some(
                        ExampleManyToManyModel {
                            id: Uuid::new_v4(),
                            name: String::from("Example 2"),
                            created_at: Utc::now().naive_utc(),
                            updated_at: Utc::now().naive_utc(),
                        }
                    )
                ),
            ]
        ])
            .into_connection();

            let mut repository = setup_repository(&connection);

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
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::RecordNotFound(String::from("Not found"))])
            .into_connection();

            let mut repository = setup_repository(&connection);

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_result = repository.get_one(id_str).await;

            assert!(example_result.is_err());
            assert_eq!(example_result.err().unwrap().error_type, RepositoryErrorType::NotFound);

        Ok(())
    }

    #[async_std::test]
    async fn test_get_one_error() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::RecordNotInserted])
            .into_connection();

            let mut repository = setup_repository(&connection);

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_result = repository.get_one(id_str).await;

            assert!(example_result.is_err());
            assert_eq!(example_result.err().unwrap().error_type, RepositoryErrorType::Unknown);

        Ok(())
    }

    #[async_std::test]
    async fn test_create() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([
                MockExecResult {
                    last_insert_id: 1,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 2,
                    rows_affected: 1,
                },
                MockExecResult {
                    last_insert_id: 3,
                    rows_affected: 1,
                },
            ])
            .append_query_results([
                vec![
                    (
                        ExampleSeaOrmWithRelationModel {
                            id,
                            example_id: Uuid::new_v4(),
                            created_at: Utc::now().naive_utc(),
                            updated_at: Utc::now().naive_utc(),
                        },
                        Some(
                            ExampleManyToManyModel {
                                id: Uuid::new_v4(),
                                name: String::from("Example 1"),
                                created_at: Utc::now().naive_utc(),
                                updated_at: Utc::now().naive_utc(),
                            }
                        )
                    ),
                    (
                        ExampleSeaOrmWithRelationModel {
                            id,
                            example_id: Uuid::new_v4(),
                            created_at: Utc::now().naive_utc(),
                            updated_at: Utc::now().naive_utc(),
                        },
                        Some(
                            ExampleManyToManyModel {
                                id: Uuid::new_v4(),
                                name: String::from("Example 2"),
                                created_at: Utc::now().naive_utc(),
                                updated_at: Utc::now().naive_utc(),
                            }
                        )
                    ),
                ]
            ])
            .into_connection();
    
        let mut repository = setup_repository(&connection);
    
        let example_create_dto = ExampleWithRelationCreateDto {
            example_id: Some(Uuid::new_v4().to_string()),
            example_many_to_manys: Some(vec![
                ExampleManyToManyCreateDto {
                    name: Some(String::from("Example 1")),
                },
                ExampleManyToManyCreateDto {
                    name: Some(String::from("Example 2")),
                },
            ]),
        };
    
        let example_create_result = repository.create(example_create_dto).await;
    
        let example = example_create_result.ok().unwrap();
    
        assert!(example.id.len() > 0);
    
        Ok(())
    }

    /* #[async_std::test]
    async fn test_create_error() -> Result<(), DbErr> {
        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_errors(vec![DbErr::RecordNotInserted])
            .into_connection();

            let mut repository = setup_repository(&connection);

            let example_create_dto = ExampleCreateDto {
                name: Some(String::from("Example 1"))
            };

            let example_create_result = repository.create(example_create_dto).await;

            assert!(example_create_result.is_err());
            assert_eq!(example_create_result.err().unwrap().error_type, RepositoryErrorType::Unknown);

        Ok(())
    }

    #[async_std::test]
    async fn test_update() -> Result<(), DbErr> {
        let id = Uuid::new_v4();

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
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

            let mut repository = setup_repository(&connection);

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

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::RecordNotFound(String::from("Not found"))])
            .into_connection();

            let mut repository = setup_repository(&connection);

            let example_update_dto = ExampleUpdateDto {
                name: Some(String::from("Example 1"))
            };

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_update_result = repository.update(id_str, example_update_dto).await;

            assert!(example_update_result.is_err());

            assert_eq!(example_update_result.err().unwrap().error_type, RepositoryErrorType::NotFound);

        Ok(())
    }

    #[async_std::test]
    async fn test_update_get_error() -> Result<(), DbErr> {
        let id = Uuid::new_v4();

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_errors(vec![DbErr::RecordNotUpdated])
            .into_connection();

            let mut repository = setup_repository(&connection);

            let example_update_dto = ExampleUpdateDto {
                name: Some(String::from("Example 1"))
            };

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_update_result = repository.update(id_str, example_update_dto).await;

            assert!(example_update_result.is_err());

            assert_eq!(example_update_result.err().unwrap().error_type, RepositoryErrorType::Unknown);

        Ok(())
    }

    #[async_std::test]
    async fn test_update_error() -> Result<(), DbErr> {
        let id = Uuid::new_v4();

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

            let mut repository = setup_repository(&connection);

            let example_update_dto = ExampleUpdateDto {
                name: Some(String::from("Example 1"))
            };

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_update_result = repository.update(id_str, example_update_dto).await;

            assert!(example_update_result.is_err());

            assert_eq!(example_update_result.err().unwrap().error_type, RepositoryErrorType::Unknown);

        Ok(())
    }

    #[async_std::test]
    async fn test_delete() -> Result<(), DbErr> {
        let id = Uuid::new_v4();

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([
                MockExecResult {
                    rows_affected: 1,
                    last_insert_id: 0
                }
            ])
            .into_connection();

            let mut repository = setup_repository(&connection);

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_delete_result = repository.delete(id_str).await;

            assert!(example_delete_result.is_ok());

        Ok(())
    }

    #[async_std::test]
    async fn test_delete_not_found() -> Result<(), DbErr> {
        let id = Uuid::new_v4();

        let connection = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([
                MockExecResult {
                    rows_affected: 0,
                    last_insert_id: 0
                }
            ])
            .into_connection();

            let mut repository = setup_repository(&connection);

            let binding = id.to_string();
            let id_str = binding.as_str();

            let example_delete_result = repository.delete(id_str).await;

            assert!(example_delete_result.is_err());

            assert_eq!(example_delete_result.err().unwrap().error_type, RepositoryErrorType::NotFound);

        Ok(())
    } */
}