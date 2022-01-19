use crate::{common::error::Error, modules::enums::OS};
use actix_web::{delete, get, post, put, HttpRequest, HttpResponse};
use chrono::{DateTime, Local, Utc};
use sqlx::MySqlPool;
use serde::Serialize;

// #[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
// #[sqlx(rename = "color")] // May also be the name of a user defined enum type
// #[sqlx(rename_all = "lowercase")] // similar to serde rename_all
// pub enum Color {
//     Red,
//     Green,
//     Blue,
// } // expects 'red', 'green', or 'blue'

pub mod database {
    use actix_web::{
        delete, get, post, put,
        web::{Data, Json},
        HttpRequest, HttpResponse,
    };
    use chrono::{DateTime, Local, Utc};
    use log::info;
    use serde::{Deserialize, Serialize};
    use sqlx::{Executor, FromRow, MySqlPool, Type};
    use derive_more::Display;

    use crate::{
        common::{app_state::AppState, error::Error},
        modules::enums::OS,
    };

    #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
    pub struct Media {
        pub id: u64,
        pub created_at: DateTime<Local>,
        pub updated_at: DateTime<Local>,

        pub name: String,
        pub desc: Option<String>,

        pub media_type: MediaType,
        pub site_url: Option<String>,
        pub os: Option<OS>,
        pub package_name: Option<String>,
        pub down_load_url: Option<String>,
    }

    #[derive(Debug, Clone, Display, Serialize, Deserialize, Type)]
    #[sqlx(rename = "media_type")]
    pub enum MediaType {
        WAP,
        APP,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct PostRequest {
        pub name: String,
        pub desc: Option<String>,

        pub media_type: MediaType,

        pub site_url: Option<String>,

        pub os: Option<OS>,
        pub package_name: Option<String>,
        pub down_load_url: Option<String>,
    }

	impl PostRequest {
		fn self_check(&self) -> Result<(), Error> {
			match self.media_type {
				MediaType::WAP => {
					if self.site_url == None {
						return Err(Error::BadRequest(400, "media_type == 'WAP', 'site_url' can't be null"));
					}
				},
				MediaType::APP => {
					if self.os == None {
						return Err(Error::BadRequest(400, "media_type == 'APP', 'os' can't be null"));
					} else if self.package_name == None {
						return Err(Error::BadRequest(400, "media_type == 'APP', 'package_name' can't be null"));
					} else if self.down_load_url == None {
						return Err(Error::BadRequest(400, "media_type == 'APP', 'down_load_url' can't be null"));
					}
				}
			};
			Ok(())
		}
	}

    #[post("/media")]
    pub async fn post(
        state: Data<AppState>,
        req: Json<PostRequest>,
    ) -> Result<HttpResponse, Error> {
		req.self_check()?;

        let id = sqlx::query(r#"insert into media (name, `desc`, media_type, site_url, os, package_name, down_load_url) values (?, ?, ?, ?, ?, ?, ?)"#)
			.bind(&req.name)
			.bind(&req.desc)
			.bind(&req.media_type)
			.bind(&req.site_url)
			.bind(&req.os)
			.bind(&req.package_name)
			.bind(&req.down_load_url)
			.execute(&state.pool)
			.await?
			.last_insert_id();
        Ok(HttpResponse::Ok().json(get_by_id(&state.pool, id).await?))
    }

    pub async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Media>, Error> {
		let media = sqlx::query_as!(Media, r#"
			select id, created_at as "created_at: _", updated_at as "updated_at: _", name, `desc`, media_type as "media_type: MediaType", site_url, os as "os: OS", package_name, down_load_url
				from media where id = ?"#, id)
			.fetch_optional(pool) 
			.await?;
        info!("media: {:?}", media);
        Ok(media)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Media {
    pub id: u64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,

    pub name: String,
    pub desc: Option<String>,
    pub media_type: MediaType,
}

#[derive(Debug, Clone, Serialize)]
pub enum MediaType {
    Wap {
        site_url: String,
    },
    App {
        os: OS,
        package_name: String,
        down_load_url: String,
    },
}

impl From<database::Media> for Media {
    fn from(media: database::Media) -> Self {
        Media {
            id: media.id,
            created_at: media.created_at.clone(),
            updated_at: media.updated_at.clone(),
            name: media.name.clone(),
            desc: media.desc.clone(),
            media_type: match media.media_type {
                database::MediaType::APP => MediaType::App {
                    os: media.os.unwrap().clone(),
                    package_name: media.package_name.unwrap().clone(),
                    down_load_url: media.down_load_url.unwrap().clone(),
                },
                database::MediaType::WAP => MediaType::Wap {
                    site_url: media.site_url.unwrap().clone(),
                },
            },
        }
    }
}

pub async fn get_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Media>, Error> {
	Ok(database::get_by_id(pool, id).await?.map_or(None, |media| Some(media.into())))
}
