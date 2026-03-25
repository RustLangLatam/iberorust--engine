use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "chapters")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub module_id: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub is_quiz: Option<bool>,
    pub order: i32,
    #[sea_orm(column_type = "Text")]
    pub video_url: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::module::Entity",
        from = "Column::ModuleId",
        to = "super::module::Column::Id"
    )]
    Module,
}

impl Related<super::module::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Module.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
