use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "progress")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub chapter_id: Uuid,
    pub completed: Option<bool>,
    pub score: Option<i32>,
    pub completed_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::chapter::Entity",
        from = "Column::ChapterId",
        to = "super::chapter::Column::Id"
    )]
    Chapter,
}

impl Related<super::chapter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chapter.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
