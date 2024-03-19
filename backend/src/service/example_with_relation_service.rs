use sea_orm::{ActiveModelBehavior, ActiveModelTrait, EntityTrait, IntoActiveModel};
use uuid::Uuid;
use validator::{HasLen, Validate};

use crate::{dto::{example_dto::ExampleOrderDto, example_with_relation_dto::{ExampleWithRelationCreateDto, ExampleWithRelationGetDto, ExampleWithRelationUpdateDto}, list_options_dto::ListOptionsDto}, repository::{base_seaorm_repository::BaseSeaOrmRepository, example_with_relation_repository::ExampleWithRelationRepository}, status::ResponseStatus, transformer::sea_orm_transformer::SeaOrmTransformer};

use super::{error::{to_api_error, ApiError, ApiValidationError, NotFoundError, UnknownError}, result::{SuccessCreateResult, SuccessGetManyResult, SuccessGetOneResult, SuccessUpdateResult}};

pub struct ExampleWithRelationService<'a> {
    pub repository: ExampleWithRelationRepository<'a>
}

impl<'a> ExampleWithRelationService<'a> {

    pub async fn get_many(&mut self, options: ListOptionsDto<ExampleOrderDto>) -> Result<SuccessGetManyResult<ExampleWithRelationGetDto>, Box<dyn ApiError<'a> + 'a>> {
        let repository_result = self.repository.get_many(options).await;
        
        match repository_result {
            Err(rep_error) => Err(to_api_error(rep_error)),
            Ok(items) => Ok(SuccessGetManyResult {
                status: ResponseStatus::Success,
                items: items.clone(),
                total_count: items.length(),
                num_pages: None
            })
        }
        
    }

    pub async fn get_one(&mut self, example_id: &'a str) -> Result<SuccessGetOneResult<ExampleWithRelationGetDto>, Box<dyn ApiError<'a> + 'a>> {
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

    pub async fn create(&mut self, data: ExampleWithRelationCreateDto) -> Result<SuccessCreateResult<ExampleWithRelationGetDto>, Box<dyn ApiError<'a> + 'a>> {
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

    pub async fn update(&mut self, id: &'a str, data: ExampleWithRelationUpdateDto) -> Result<SuccessUpdateResult<ExampleWithRelationGetDto>, Box<dyn ApiError<'a> + 'a>> {
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

    pub async fn delete(&mut self, id: &'a str) -> Result<(), Box<dyn ApiError<'a> + 'a>> {
        let delete_result = self.repository.delete(id).await;

        match delete_result {
            Err(rep_err) => Err(to_api_error(rep_err)),
            Ok(()) => Ok(())
        }
    }
}