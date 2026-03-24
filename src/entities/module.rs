use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "modules")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub description: Option<String>,
    pub order: i32,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::course::Entity",
        from = "Column::CourseId",
        to = "super::course::Column::Id"
    )]
    Course,
}

impl Related<super::course::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
