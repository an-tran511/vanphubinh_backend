use discern::{query::QueryBus, query_bus};
use sea_orm::DatabaseConnection;
use service::{
  list_paginated_categories_query::{ListCategoriesQuery, ListCategoriesQueryHandler},
  measurement::query::{
    find_uom_by_id_query::{FindUomByIdQuery, FindUomByIdQueryHandler},
    list_paginated_uoms_query::{ListUomsQuery, ListUomsQueryHandler},
  },
};
use std::sync::Arc;

pub fn new(read_db: Arc<DatabaseConnection>) -> QueryBus {
  query_bus! {
    //Measurement service queries
    ListUomsQuery => ListUomsQueryHandler { db: read_db.clone() },
    FindUomByIdQuery => FindUomByIdQueryHandler { db: read_db.clone() },

    //Catalog service queries
    ListCategoriesQuery => ListCategoriesQueryHandler { db: read_db.clone() },
  }
}
