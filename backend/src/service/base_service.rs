use sea_orm::{ActiveModelBehavior, ActiveModelTrait, EntityTrait, IntoActiveModel};
use uuid::Uuid;
use validator::Validate;

use crate::{dto::list_options_dto::ListOptionsDto, repository::base_seaorm_repository::BaseSeaOrmRepository, status::ResponseStatus, transformer::sea_orm_transformer::SeaOrmTransformer};

use super::{error::{to_api_error, ApiError, ApiValidationError, NotFoundError, UnknownError}, result::{SuccessCreateResult, SuccessGetManyResult, SuccessGetOneResult, SuccessUpdateResult}};

pub struct BaseService<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, EntityOrderDto, Transformer, AM> {
    pub repository: BaseSeaOrmRepository<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, EntityOrderDto, Transformer, AM>
}

impl<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, EntityOrderDto, Transformer, AM> 
    BaseService<'a, SeaOrmModel, GetModelDto, CreateModelDto, UpdateModelDto, EntityOrderDto, Transformer, AM>  where 
    SeaOrmModel: EntityTrait,
    CreateModelDto: Validate,
    UpdateModelDto: Validate,
    EntityOrderDto: Validate,
    Transformer: SeaOrmTransformer<'a, GetModelDto, CreateModelDto, UpdateModelDto, EntityOrderDto, SeaOrmModel, AM>,
    AM: ActiveModelBehavior + std::marker::Send {

    pub async fn get_many(&mut self, options: ListOptionsDto<EntityOrderDto>) -> Result<SuccessGetManyResult<GetModelDto>, Box<dyn ApiError<'a> + 'a>> where <SeaOrmModel as sea_orm::EntityTrait>::Model: Sync {
        let validation_result = options.validate();
        if validation_result.is_err() {
            return Err(Box::new(ApiValidationError::new(None, validation_result.err())))
        }
        
        let repository_result = self.repository.get_many(options).await;
        
        match repository_result {
            Err(rep_error) => Err(to_api_error(rep_error)),
            Ok(res) => Ok(SuccessGetManyResult {
                status: ResponseStatus::Success,
                items: res.items,
                total_count: res.total_count,
                num_pages: res.num_pages
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

    pub async fn update(&mut self, id: &'a str, data: UpdateModelDto) -> Result<SuccessUpdateResult<GetModelDto>, Box<dyn ApiError<'a> + 'a>> where 
    <<AM as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Model: IntoActiveModel<AM>, 
    <<SeaOrmModel as sea_orm::EntityTrait>::PrimaryKey as sea_orm::PrimaryKeyTrait>::ValueType: From<uuid::Uuid> {
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