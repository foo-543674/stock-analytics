use axum::Json;
use serde::Deserialize;
use crate::applications::{
  brand::{
    interactors::brand_register_input::BrandRegisterInput, 
    usecases::register_brand_usecase::RegisterBrandUsecase
  },
  errors::application_error::ApplicationError
};
use super::brand_json::BrandJson;

#[derive(Deserialize)]
pub struct PostBrandBody {
  name: String,
  code: String,
  sector_id: String
}

impl PostBrandBody {
  fn to_input(&self) -> BrandRegisterInput {
    BrandRegisterInput {
      name: self.name.clone(),
      code: self.code.clone(),
      sector_id: self.sector_id.clone()
    }
  }
}

pub struct PostBrandHandler {
  usecase: RegisterBrandUsecase,
}

impl PostBrandHandler {
  pub fn new(usecase: RegisterBrandUsecase) -> Self {
    Self { usecase }
  }

  pub const PATH: &str = "/brands";

  pub async fn handle(&self, Json(payload): Json<PostBrandBody>) -> Result<Json<BrandJson>, ApplicationError> {
    let input = payload.to_input();
    let result = self.usecase.execute(input).await?;
    Ok(Json(BrandJson::from_brand(result)))
  }
}
