//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.8

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "account_api_plans")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub max_req_per_second: Option<i16>,
    pub name: String,
    pub inserted_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::account_identities::Entity")]
    AccountIdentities,
}

impl Related<super::account_identities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccountIdentities.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
