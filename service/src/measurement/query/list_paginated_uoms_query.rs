use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use discern::async_trait;
use discern::query::Query;
use discern::query::QueryHandler;
use domain::uom::UomDTO;
use domain::uom::{Column, Entity as Uom};
use infra::util::{error, PaginationMeta};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QuerySelect};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ListUomsQuery {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

#[derive(Error, Debug)]
pub enum ListUomsError {
  #[error("internal_server_error")]
  InternalServerError(#[from] DbErr),
}

impl IntoResponse for ListUomsError {
  fn into_response(self) -> Response {
    let (status, code) = match self {
      ListUomsError::InternalServerError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
      }
    };

    (
      status,
      error(code, Some("list_paginated_uoms_query".to_string())),
    )
      .into_response()
  }
}

#[derive(Debug, Serialize)]
pub struct ListUomsQueryOutput {
  pub data: Vec<UomDTO>,
  pub meta: PaginationMeta,
}

// Implement the Query trait for ListUomsQuery.
impl Query for ListUomsQuery {
  type Output = ListUomsQueryOutput; // Return the user as output.
  type Error = ListUomsError;
}

// Define a handler for the ListUomsQuery.
pub struct ListUomsQueryHandler {
  pub db: Arc<DatabaseConnection>,
}

#[async_trait]
impl QueryHandler<ListUomsQuery> for ListUomsQueryHandler {
  async fn handle(&self, query: ListUomsQuery) -> Result<ListUomsQueryOutput, ListUomsError> {
    let per_page = query.per_page.unwrap_or(30);
    let page = query.page.unwrap_or(1) - 1;

    let uom_pages = Uom::find()
      .select_only()
      .column(Column::Id)
      .column(Column::Name)
      .into_partial_model::<UomDTO>()
      .paginate(self.db.as_ref(), per_page);
    let uoms = uom_pages.fetch_page(page).await?;
    let items_and_pages = uom_pages.num_items_and_pages().await?;
    let total = items_and_pages.number_of_items;
    let total_pages = items_and_pages.number_of_pages;

    Ok(ListUomsQueryOutput {
      data: uoms,
      meta: PaginationMeta {
        page: page + 1,
        total_pages,
        per_page,
        total,
      },
    })
  }
}
