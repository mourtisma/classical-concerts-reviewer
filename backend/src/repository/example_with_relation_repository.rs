use std::{marker::PhantomData, vec};

use sea_orm::{sea_query::Table, ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set, SqlErr, TransactionTrait, TryIntoModel};
use uuid::Uuid;
use crate::{dto::example_with_relation_dto::{ExampleWithRelationCreateDto, ExampleWithRelationGetDto}, model::prelude::*, transformer::{example_with_relation_transformer::ExampleWithRelationTransformer, sea_orm_transformer::SeaOrmTransformer}};
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

    /* pub async fn get_one(&mut self, item_id: &'a str) -> Result<GetModelDto, RepositoryError<'a>> 
    where <<SeaOrmModel as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType: From<Uuid> {
        let get_one_result = SeaOrmModel::find_by_id(Uuid::parse_str(item_id).unwrap()).one(self.connection).await;
        
        if let Err(get_one_error) = get_one_result {
            match get_one_error {
                DbErr::RecordNotFound(_) => Err(RepositoryError { //TODO use the message arg for logs
                    error_type: RepositoryErrorType::NotFound,
                    message: Some("Record was not found"),
                    orm_error: Some(get_one_error)
                }),
                _ => Err(RepositoryError {
                    error_type: RepositoryErrorType::Unknown,
                    message: Some("An unknow error occurred"),
                    orm_error: Some(get_one_error)
                })
            }
        } else if let Ok(item) = get_one_result {
            match(item) {
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
        
    } */

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
                
                Ok(example_with_relation_entity_id)

                
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
        } else if let Ok(new_entity_id) = insert_result {
            let new_entity = &ExampleSeaOrmWithRelation::find_by_id(new_entity_id).find_with_related(ExampleManyToMany).all(self.connection).await.unwrap()[0];
            Ok(ExampleWithRelationTransformer::entity_to_get_dto(new_entity.clone()))
        } else {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                orm_error: None
            })
        }
    }

     /* pub async fn update(&mut self, item_id: &'a str, data: UpdateModelDto) -> Result<GetModelDto, RepositoryError<'a>> where 
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
                        orm_error: Some(update_error)
                    }),
                    _ => Err(RepositoryError {
                        error_type: RepositoryErrorType::Unknown,
                        message: Some("An unknow error occurred"),
                        orm_error: Some(update_error)
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
        
    }*/

      

}