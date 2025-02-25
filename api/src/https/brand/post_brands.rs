use super::brand_json::BrandJson;
use crate::applications::{
  brand::{
    interactors::brand_register_input::BrandRegisterInput,
    usecases::register_brand_usecase::RegisterBrandUsecase,
  },
  errors::application_error::ApplicationError,
};
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostBrandBody {
  name: String,
  code: String,
  sector_id: String,
}

impl PostBrandBody {
  fn to_input(&self) -> BrandRegisterInput {
    BrandRegisterInput {
      name: self.name.clone(),
      code: self.code.clone(),
      sector_id: self.sector_id.clone(),
    }
  }
}

pub async fn post_brands(
  Json(payload): Json<PostBrandBody>,
  usecase: &RegisterBrandUsecase,
) -> Result<Json<BrandJson>, ApplicationError> {
  let input = payload.to_input();
  let result = usecase.execute(input).await?;
  Ok(Json(BrandJson::from_brand(&result)))
}
