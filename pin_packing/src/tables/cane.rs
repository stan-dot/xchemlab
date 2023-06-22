use super::puck;
use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationTrait,
};
use uuid::Uuid;

pub const CANE_SLOTS: i16 = 7;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "cane")]
#[graphql(name = "Cane", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub barcode: Uuid,
    #[sea_orm(primary_key)]
    pub created: DateTime<Utc>,
    pub operator_id: String,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "puck::Entity")]
    Puck,
}

impl Related<puck::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Puck.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}