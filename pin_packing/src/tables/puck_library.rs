use super::puck_mount;
use async_graphql::{Enum, SimpleObject};
use axum::async_trait;
use sea_orm::{
    ActiveModelBehavior, DeriveActiveEnum, DeriveEntityModel, DerivePrimaryKey, DeriveRelation,
    EntityTrait, EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Enum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "puck_status")]
pub enum PuckStatus {
    #[sea_orm(string_value = "Ready")]
    Ready,
    #[sea_orm(string_value = "Filling")]
    Filling,
    #[sea_orm(string_value = "Away")]
    Away,
    #[sea_orm(string_value = "Broken")]
    Broken,
}

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "puck_library")]
#[graphql(name = "LibraryPuck")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub barcode: String,
    pub status: PuckStatus,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "puck_mount::Entity")]
    PuckMount,
}

impl Related<puck_mount::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::PuckMount.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
