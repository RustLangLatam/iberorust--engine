use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "thread_likes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub thread_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
