use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use discern::async_trait;
use discern::query::Query;
use discern::query::QueryHandler;
use domain::category::CategoryDTO;
use domain::category::{Column, Entity as Category};
use infra::response::PaginationMeta;
use infra::util::error;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QuerySelect};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListCategoriesQuery {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

#[derive(Error, Debug)]
pub enum ListCategoriesError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for ListCategoriesError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      ListCategoriesError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (
      status,
      error(code, Some("list_paginated_categories_query".to_string())),
    )
      .into_response()
  }
}

#[derive(Debug, Serialize)]
pub struct ListCategoriesQueryOutput {
  pub data: Vec<CategoryDTO>,
  pub meta: PaginationMeta,
}

// Implement the Query trait for ListUomsQuery.
impl Query for ListCategoriesQuery {
  type Output = ListCategoriesQueryOutput; // Return the user as output.
  type Error = ListCategoriesError;
}

pub struct ListCategoriesQueryHandler {
  pub db: Arc<DatabaseConnection>,
}

#[async_trait]
impl QueryHandler<ListCategoriesQuery> for ListCategoriesQueryHandler {
  async fn handle(
    &self,
    query: ListCategoriesQuery,
  ) -> Result<ListCategoriesQueryOutput, ListCategoriesError> {
    let per_page = query.per_page.unwrap_or(30);
    let page = query.page.unwrap_or(1) - 1;

    let category_pages = Category::find()
      .select_only()
      .column(Column::Id)
      .column(Column::Name)
      .into_partial_model::<CategoryDTO>()
      .paginate(self.db.as_ref(), per_page);
    let categories = category_pages.fetch_page(page).await?;
    let items_and_pages = category_pages.num_items_and_pages().await?;
    let total = items_and_pages.number_of_items;
    let total_pages = items_and_pages.number_of_pages;

    Ok(ListCategoriesQueryOutput {
      data: categories,
      meta: PaginationMeta {
        page: page + 1,
        total_pages,
        per_page,
        total,
      },
    })
  }
}
