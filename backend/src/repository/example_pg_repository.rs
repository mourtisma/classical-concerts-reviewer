use std::{marker::PhantomData, vec};

use sea_orm::{sea_query::Table, ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, SqlErr, TryIntoModel};
use uuid::Uuid;
use crate::{model::prelude::*, transformer::sea_orm_transformer::SeaOrmTransformer};
use super::{error::{RepositoryError, RepositoryErrorType}, list_options::ListOptions};

pub struct ExamplePgRepository<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModel, Transformer, AM> {
    pub connection: &'a DatabaseConnection,
    pub _phantom_lifetime: PhantomData<&'a String>,
    pub _phantom_sea_orm: PhantomData<SeaOrmModel>,
    pub _phantom_get: PhantomData<GetModelDto>,
    pub _phantom_create: PhantomData<CreateModelDto>,
    pub _phantom_update: PhantomData<UpdateModel>,
    pub _phantom_transformer: PhantomData<Transformer>,
    pub _phantom_active_model: PhantomData<AM>
}


impl<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, Transformer, AM> 
    ExamplePgRepository<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, Transformer, AM> where 
        SeaOrmModel: EntityTrait,
        Transformer: SeaOrmTransformer<'a, GetModelDto, CreateModelDto, UpdateModelDto, SeaOrmModel, AM>,
        AM: ActiveModelBehavior + std::marker::Send, {

    pub async fn get_many(&mut self, options: ListOptions) -> Result<Vec<GetModelDto>, RepositoryError<'a>> {
        
        let get_many_result = SeaOrmModel::find().all(self.connection).await;
        
        if get_many_result.is_err() {
            Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                orm_error: get_many_result.err()
            })
        } else {
            match get_many_result.ok() {
                Some(items_list) => Ok(items_list.iter().map(|it| Transformer::entity_to_get_dto(it.clone())).collect()),
                None => Ok(vec![]),    
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
        
    }

    pub async fn create(&mut self, data: CreateModelDto) -> Result<GetModelDto, RepositoryError<'a>> where <<AM as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<AM> {
        let active_model = Transformer::dto_to_create_active_model(data);
        let insert_result = active_model.insert(self.connection).await;
        
        match insert_result {
            Err(orm_error) => Err(RepositoryError {
                error_type: RepositoryErrorType::Unknown,
                message: Some("An unknow error occurred"),
                orm_error: Some(orm_error)
            }),
            Ok(new_item) => Ok(Transformer::active_model_to_dto(new_item))
        }
    }

    pub async fn update(&mut self, item_id: &'a str, data: UpdateModelDto) -> Result<GetModelDto, RepositoryError<'a>> where <<AM as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<AM> {
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
        
    }

      

}