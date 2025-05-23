//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.8

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "custom(\"citext\")", unique)]
    pub username: String,
    pub password_hash: String,
    pub inserted_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::administrators::Entity")]
    Administrators,
    #[sea_orm(has_one = "super::user_contacts::Entity")]
    UserContacts,
}

impl Related<super::administrators::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Administrators.def()
    }
}

impl Related<super::user_contacts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserContacts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
