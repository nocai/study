mod admin;
pub mod enums;
mod ssp;
mod weight;

use actix_web::web::ServiceConfig;

pub fn route(cfg: &mut ServiceConfig) {
    cfg.service(admin::admin::get);
    cfg.service(admin::admin::post);
    cfg.service(admin::admin::put);
    cfg.service(admin::admin::delete);

    cfg.service(admin::role::post);
    cfg.service(admin::role::get);
    cfg.service(admin::role::put);
    cfg.service(admin::role::delete);

    cfg.service(ssp::media::database::post);
    cfg.service(ssp::slot::post);

    cfg.service(ssp::ssp);
}
