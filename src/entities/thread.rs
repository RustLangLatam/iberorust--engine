use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "threads")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    // Note: SeaORM maps Postgres text[] to Vec<String> when the `postgres-array` feature is enabled.
    // The macro will infer the column type if it's a simple array.
    pub tags: Option<Vec<String>>,
    pub likes_count: Option<i32>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
