mod admin;

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
}
