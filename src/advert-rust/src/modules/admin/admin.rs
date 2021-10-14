use crate::common::{app_state::AppState, error::Error};
use actix_web::{
    delete, get,
    http::StatusCode,
    post, put,
    web::{Data, Json, Query},
    HttpRequest, HttpResponse,
};
use chrono::{DateTime, Local};
use log::{error, info};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use super::role::{self, Role};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Admin {
    pub id: u64,
    pub role_id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,

    pub username: String,
    pub password: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct GetAdminRequest {
    pub admin_id: Option<u64>,
    pub role_id: Option<u64>,
    pub began_at: Option<DateTime<Local>>,
    pub ended_at: Option<DateTime<Local>>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetAdminResponse {
    pub admin: Admin,
    pub role: Option<Role>,
}

#[get("/admins")]
pub async fn get(
    state: Data<AppState>,
    req: Query<GetAdminRequest>,
) -> Result<HttpResponse, Error> {
    let admins = do_get(&state, &req).await?;
    Ok(HttpResponse::Ok().json(admins))
}

pub async fn do_get(
    state: &AppState,
    req: &GetAdminRequest,
) -> Result<Vec<GetAdminResponse>, Error> {
    let mut sql = String::from("select * from admin where 1=1");
    if let Some(admin_id) = req.admin_id {
        sql.push_str(format!(" and id = {}", admin_id).as_str());
    }
    if let Some(role_id) = req.role_id {
        sql.push_str(format!(" and role_id = {}", role_id).as_str());
    }
    if let Some(began_at) = req.began_at {
        sql.push_str(format!(" and created_at >= {:?}", began_at.naive_utc().to_string()).as_str());
    }
    if let Some(ended_at) = req.ended_at {
        sql.push_str(format!(" and created_at <= {:?}", ended_at.naive_utc().to_string()).as_str());
    }
    if let Some(ref username) = req.username {
        sql.push_str(format!(" and username = {:?}", username).as_str());
    }

    let admins = sqlx::query_as::<_, Admin>(sql.as_str())
        .fetch_all(&state.pool)
        .await?;
    let mut response = Vec::new();
    for admin in admins {
        let role = role::get_by_id(&state.pool, admin.role_id)
            .await
            // .map_or(None, |role| Some(role));
            .map_or_else(
                |e| {
                    error!("role::get_by_id err: {:?}", e.to_string());
                    None
                },
                |v| Some(v),
            );
        info!("role: {:?}", role);
        response.push(GetAdminResponse { admin, role })
    }
    Ok(response)
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostAdminRequest {
    pub role_id: u64,
    pub username: String,
    pub password: String,
}

#[post("/admins")]
pub async fn post(
    state: Data<AppState>,
    req: Json<PostAdminRequest>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(do_post(&state, &req).await?))
}

async fn do_post(state: &AppState, req: &PostAdminRequest) -> Result<Admin, Error> {
    let id: u64 = sqlx::query("insert into admin (role_id, username, password) values (?, ?, ?)")
        .bind(req.role_id)
        .bind(req.username.to_string())
        .bind(req.password.to_string())
        .execute(&state.pool)
        .await?
        .last_insert_id();

    Ok(get_by_id(&state.pool, id).await?)
}

async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<Admin, Error> {
    let admin = sqlx::query_as::<_, Admin>("select * from admin where id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(admin)
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutAdminRequest {
    pub role_id: Option<u64>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[put("/admins/{id}")]
pub async fn put(
    state: Data<AppState>,
    req: Json<PutAdminRequest>,
    req2: HttpRequest,
) -> Result<HttpResponse, Error> {
    let mut sql = String::from("update admin set id = id");
    if let Some(role_id) = req.role_id {
        sql.push_str(format!(", role_id = {}", role_id).as_str());
    }
    if let Some(ref username) = req.username {
        sql.push_str(format!(", username = {:?}", username).as_str());
    }
    if let Some(ref password) = req.password {
        sql.push_str(format!(", password = {:?}", password).as_str());
    }
    let id: u64 = req2.match_info().query("id").parse()?;
    sql.push_str(format!(" where id = {}", id).as_str());

    let rows = sqlx::query(sql.as_str())
        .execute(&state.pool)
        .await?
        .rows_affected();
    info!("rows: {}", rows);

    let admin = sqlx::query_as::<_, Admin>("select * from admin where id = ?")
        .bind(id)
        .fetch_one(&state.pool)
        .await?;
    Ok(HttpResponse::Ok().json(admin))
}

#[delete("admins/{id}")]
pub async fn delete(state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let id: u64 = req.match_info().query("id").parse()?;
    let rows = sqlx::query("delete from admin where id = ?")
        .bind(id)
        .execute(&state.pool)
        .await?
        .rows_affected();
    info!("delete rows: {}", rows);
    Ok(HttpResponse::new(StatusCode::OK))
}
