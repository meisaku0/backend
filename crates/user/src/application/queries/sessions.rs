use rocket::serde::json::Json;
use sea_orm::prelude::Uuid;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use shared::responses::error::Error;

use crate::domain::entities::SessionEntity;
use crate::domain::entities::SessionEntity::SessionMinimal;
use crate::infrastructure::http::guards::auth::JwtGuard;
use crate::presentation::dto::pagination::ItemPaginationDTO;
use crate::presentation::dto::sessions::UserSessionPaginateDTO;

pub async fn action(
    conn: &DatabaseConnection, jwt_guard: JwtGuard, user_session_paginate: UserSessionPaginateDTO,
) -> Result<Json<ItemPaginationDTO>, Error> {
    let sessions = SessionEntity::Entity::find()
        .order_by_desc(SessionEntity::Column::CreatedAt)
        .filter(SessionEntity::Column::UserId.eq(Uuid::parse_str(&jwt_guard.claims.sub).unwrap()))
        .filter(SessionEntity::Column::Active.eq(true))
        .filter(SessionEntity::Column::Browser.contains(user_session_paginate.browser.unwrap_or_default()))
        .filter(SessionEntity::Column::Device.contains(user_session_paginate.device.unwrap_or_default()))
        .filter(SessionEntity::Column::Ip.contains(user_session_paginate.ip.unwrap_or_default()))
        .filter(SessionEntity::Column::Os.contains(user_session_paginate.os.unwrap_or_default()))
        .into_partial_model::<SessionMinimal>()
        .paginate(conn, user_session_paginate.per_page);

    let page = user_session_paginate.page - 1;
    let pagination_data = sessions.num_items_and_pages().await?;

    Ok(Json(ItemPaginationDTO {
        items: sessions.fetch_page(page).await?,
        total_items: pagination_data.number_of_items,
        total_pages: pagination_data.number_of_pages,
        page: page + 1,
        has_previous_page: page > 1,
        has_next_page: page + 1 < pagination_data.number_of_pages,
        per_page: user_session_paginate.per_page,
    }))
}
