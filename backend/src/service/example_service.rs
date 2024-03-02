use sea_orm::{ActiveModelBehavior, ActiveModelTrait, EntityTrait, IntoActiveModel};
use uuid::Uuid;
use validator::Validate;

use crate::{repository::{example_pg_repository::ExamplePgRepository, list_options::ListOptions}, status::ResponseStatus, transformer::sea_orm_transformer::SeaOrmTransformer};

use super::{error::{to_api_error, ApiError, ApiValidationError, NotFoundError, UnknownError}, result::{SuccessCreateResult, SuccessGetManyResult, SuccessGetOneResult, SuccessUpdateResult}};

pub struct ExampleService<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, Transformer, AM> {
    pub repository: ExamplePgRepository<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, Transformer, AM>
}

impl<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, Transformer, AM> 
    ExampleService<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, Transformer, AM>  where 
    SeaOrmModel: EntityTrait,
    CreateModelDto: Validate,
    UpdateModelDto: Validate,
    Transformer: SeaOrmTransformer<'a, GetModelDto, CreateModelDto, UpdateModelDto, SeaOrmModel, AM>,
    AM: ActiveModelBehavior + std::marker::Send {

    pub async fn get_many(&mut self, options: ListOptions) -> Result<SuccessGetManyResult<GetModelDto>, Box<dyn ApiError<'a> + 'a>> {
        let repository_result = self.repository.get_many(options).await;
        
        match repository_result {
            Err(rep_error) => Err(to_api_error(rep_error)),
            Ok(items) => Ok(SuccessGetManyResult {
                status: ResponseStatus::Success,
                items
            })
        }
        
    }

    pub async fn get_one(&mut self, example_id: &'a str) -> Result<SuccessGetOneResult<GetModelDto>, Box<dyn ApiError<'a> + 'a>> where <<SeaOrmModel as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType: From<Uuid> {
        let repository_result = self.repository.get_one(example_id).await;
        
        if let Ok(item) = repository_result {
            Ok(SuccessGetOneResult {
                    status: ResponseStatus::Success,
                    item
                })
        } else if let Err(repository_error) = repository_result {
            return Err(to_api_error(repository_error))
        } else {
            return Err(Box::new(UnknownError::new(None, None)))
        }
        
    }

    pub async fn create(&mut self, data: CreateModelDto) -> Result<SuccessCreateResult<GetModelDto>, Box<dyn ApiError<'a> + 'a>> where <<AM as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<AM> {
        let validation_result = data.validate();
        if validation_result.is_err() {
            return Err(Box::new(ApiValidationError::new(None, validation_result.err())))
        }

        let repository_result = self.repository.create(data).await;

        match repository_result {
            Err(rep_error) => Err(to_api_error(rep_error)),
            Ok(item) => Ok(SuccessCreateResult {
                status: ResponseStatus::Success,
                item
            })
        }
        
    }

    pub async fn update(&mut self, id: &'a str, data: UpdateModelDto) -> Result<SuccessUpdateResult<GetModelDto>, Box<dyn ApiError<'a> + 'a>> where <<AM as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<AM> {
        let validation_result = data.validate();
        if validation_result.is_err() {
            return Err(Box::new(ApiValidationError::new(None, validation_result.err())))
        }

        let repository_result = self.repository.update(id, data).await;

        match repository_result {
            Err(rep_err) => Err(to_api_error(rep_err)),
            Ok(item) => Ok(SuccessUpdateResult {
                status: ResponseStatus::Success,
                item
            })
        }
    }

    pub async fn delete(&mut self, id: &'a str) -> Result<(), Box<dyn ApiError<'a> + 'a>> where <<SeaOrmModel as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType: From<Uuid> {
        let delete_result = self.repository.delete(id).await;

        match delete_result {
            Err(rep_err) => Err(to_api_error(rep_err)),
            Ok(()) => Ok(())
        }
    }
}