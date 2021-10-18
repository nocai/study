use chrono::{DateTime, Local};
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, Type};

use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse,
};

use super::media;
use crate::common::{app_state::AppState, error::Error};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Slot {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub media_id: u64,

    pub name: String,
    pub refresh_type: RefreshType,
    pub open_screen: bool, // 是否开屏广告位
    pub times: u8,         // 轮数，次数
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[sqlx(rename = "refresh_type")]
pub enum RefreshType {
    None,        // 不刷新
    CurrentTime, // 当次刷新
    CurrentDay,  // 当天刷新
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostRequest {
    pub media_id: u64,

    pub name: String,
    pub refresh_type: RefreshType,
    pub open_screen: bool,
    pub times: u8,
}

#[post("/slots")]
pub async fn post(state: Data<AppState>, req: Json<PostRequest>) -> Result<HttpResponse, Error> {
    // check media_id
    media::get_by_id(&state.pool, req.media_id)
        .await?
        .ok_or(Error::UnprocessableEntity(
            422,
            format!("invalid media_id: {}", req.media_id).as_str(),
        ))?;
    // check open_screen + times
    if req.open_screen && req.times == 0 {
        return Err(Error::BadRequest(
            400,
            format!("times: {} must gt 0 when open_screen is true", req.times).as_str(),
        ));
    }

    let id = sqlx::query!(
        r#"insert into slot (media_id, name, refresh_type, open_screen, times) 
			values(?, ?, ?, ?, ?)"#,
        &req.media_id,
        &req.name,
        &req.refresh_type,
        &req.open_screen,
        &req.times
    )
    .execute(&state.pool)
    .await?
    .last_insert_id();

    Ok(HttpResponse::Ok().json(get_by_id(&state.pool, id).await?))
}

pub async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Slot>, Error> {
    let slot = sqlx::query_as!(Slot, r#"
		select id, created_at as "created_at: _", updated_at as "updated_at: _", media_id, name, refresh_type as "refresh_type: RefreshType", open_screen as "open_screen: _", times
			from slot where id = ?"#, id)
			.fetch_optional(pool)
			.await?;

    info!("slot: {:?}", slot);
    Ok(slot)
}
