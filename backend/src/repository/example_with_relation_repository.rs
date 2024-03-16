use std::{marker::PhantomData, vec};

use sea_orm::{sea_query::{expr, BinOper, Expr, SimpleExpr, Table}, ActiveModelBehavior, ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect, SelectColumns, Set, SqlErr, TransactionTrait, TryIntoModel};
use uuid::Uuid;
use crate::{dto::example_with_relation_dto::{ExampleWithRelationCreateDto, ExampleWithRelationGetDto, ExampleWithRelationUpdateDto}, model::{example_many_to_many, example_sea_orm_with_relation_example_many_to_many, prelude::*}, transformer::{example_with_relation_transformer::ExampleWithRelationTransformer, sea_orm_transformer::SeaOrmTransformer}};
use super::{error::{ORMError, RepositoryError, RepositoryErrorType}, list_options::ListOptions};

pub struct ExampleWithRelationRepository<'a> {
    pub connection: &'a DatabaseConnection
}

impl<'a> ExampleWithRelationRepository<'a> {

    pub async fn get_many(&mut self, options: ListOptions) -> Result<Vec<ExampleWithRelationGetDto>, RepositoryError<'a>> {
        
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

   /*pub async fn delete(&mut self, item_id: &'a str) -> Result<(), RepositoryError<'a>> where <<SeaOrmModel as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType: From<Uuid> {
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
        
    }*/

      

}