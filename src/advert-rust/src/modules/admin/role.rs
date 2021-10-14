use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::web::Query;
use actix_web::HttpRequest;
use chrono::{DateTime, Local};
use log::info;
use serde::Deserialize;
use serde::Serialize;

use actix_web::{delete, get, post, put, HttpResponse};
use sqlx::FromRow;
use sqlx::MySqlPool;

use crate::common::app_state::AppState;
use crate::common::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostRequest {
    pub name: String,
}

#[post("/roles")]
pub async fn post(state: Data<AppState>, req: Json<PostRequest>) -> Result<HttpResponse, Error> {
    let id = sqlx::query("insert into role (name) values (?)")
        .bind(req.name.to_string())
        .execute(&state.pool)
        .await?
        .last_insert_id();

    let role = get_by_id(&state.pool, id).await?;
    Ok(HttpResponse::Ok().json(role))
}

pub async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<Role, sqlx::Error> {
    let role = sqlx::query_as::<_, Role>("select * from role where id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(role)
}

#[derive(Debug, Deserialize)]
pub struct GetRequest {
    pub name: Option<String>,
}

#[get("/roles")]
pub async fn get(state: Data<AppState>, req: Query<GetRequest>) -> Result<HttpResponse, Error> {
    let mut sql = String::from("select * from role where 1 = 1");
    if let Some(ref name) = req.name {
        sql.push_str(format!(" and name = {:?}", name).as_str());
    }

    let roles = sqlx::query_as::<_, Role>(sql.as_str())
        .fetch_all(&state.pool)
        .await?;
    Ok(HttpResponse::Ok().json(roles))
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PutRequest {
    pub name: Option<String>,
}

#[put("/roles/{id}")]
pub async fn put(
    state: Data<AppState>,
    req: Json<PutRequest>,
    req2: HttpRequest,
) -> Result<HttpResponse, Error> {
    let id: u64 = req2.match_info().query("id").parse()?;

    let mut sql = String::from("update role set id = id");
    if let Some(ref name) = req.name {
        sql.push_str(format!(", name = {:?}", name).as_str());
    }
    sql.push_str(format!(" where id = {}", id).as_str());

    let rows = sqlx::query(sql.as_str())
        .execute(&state.pool)
        .await?
        .rows_affected();
    info!("rows: {}", rows);

    let role = get_by_id(&state.pool, id).await?;
    Ok(HttpResponse::Ok().json(role))
}

#[delete("roles/{id}")]
pub async fn delete(state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let id: u64 = req.match_info().query("id").parse()?;
    let rows = sqlx::query("delete from role where id = ?")
        .bind(id)
        .execute(&state.pool)
        .await?
        .rows_affected();
    info!("rows: {}", rows);

    Ok(HttpResponse::new(StatusCode::OK))
}
