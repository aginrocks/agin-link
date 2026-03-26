use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Serialize, Deserialize, Debug, Clone, DeriveEntityModel)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[sea_orm(table_name = "links")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub slug: String,

    pub target_url: String,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    #[cfg_attr(feature = "utoipa", schema(value_type = DateTime<Utc>))]
    pub created_at: DateTimeUtc,

    #[sea_orm(default = 0)]
    pub uses: i32,
}

impl ActiveModelBehavior for ActiveModel {}
