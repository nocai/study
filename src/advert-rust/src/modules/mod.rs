mod admin;
pub mod enums;
mod ssp;

use actix_web::web::ServiceConfig;
use rand::Rng;

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

pub trait RandomWeight {
    fn weight(&self) -> u32;
}

pub fn random_by_weight<T: RandomWeight>(vec: &Vec<T>) -> &T {
    if vec.len() == 1 {
        return &vec[0];
    }

    let total: u32 = vec.iter().map(|v| v.weight()).sum();
    let rd = rand::thread_rng().gen_range(0..total);
    let mut current = 0_u32;
    for v in vec.iter() {
        current += v.weight();
        if rd < current {
            return v;
        }
    }

    panic!("unreachable")
}
