pub mod medsp;
mod wanhui;

use super::{dsp_slot::DspSlot, Ad, Request};
use crate::common::{app_state::AppState, error::Error};

use async_std::task;
use log::info;
use reqwest::StatusCode;

pub trait Dsp {
    fn dsp(&self, req: &Request, slot: &DspSlot) -> Result<Vec<Ad>, Error> {
        let client = reqwest::blocking::Client::new();

        let reqwest_request = self.new_reqwest_request(req, slot)?;
        let reqwest_response = client.execute(reqwest_request)?;

        let status = reqwest_response.status();
        if status != StatusCode::OK {
            return self.handle_reqwest_response_error(reqwest_response);
        }
        self.handle_reqwest_response(reqwest_response)
    }

    fn new_reqwest_request(
        &self,
        req: &Request,
        slot: &DspSlot,
    ) -> Result<reqwest::blocking::Request, Error> {
        unimplemented!()
    }

    fn handle_reqwest_response(
        &self,
        resp: reqwest::blocking::Response,
    ) -> Result<Vec<Ad>, Error> {
        unimplemented!()
    }

    fn handle_reqwest_response_error(
        &self,
        resp: reqwest::blocking::Response,
    ) -> Result<Vec<Ad>, Error> {
        let status = resp.status();
        let text = resp.text()?;
        info!("status: {:?}, text: {:?}", status, text);

        Err(Error::new(status.as_u16() as u32, text.as_str(), status))
    }
}

pub struct DspFactory<'a> {
    state: &'a AppState,
}

impl<'a> DspFactory<'a> {
    pub fn new(state: &'a AppState) -> DspFactory {
        DspFactory { state }
    }

    pub fn dsp(&self, provider: &str) -> Box<dyn Dsp + 'a> {
        match provider {
            "A" => Box::new(A {}),
            "WanHui" => Box::new(wanhui::WanHuiDsp::new(self.state)),
            _ => Box::new(medsp::MeDsp::new(self.state, provider)),
        }
    }
}

pub struct A {}

impl Dsp for A {
    // fn dsp(&self, req: &Request) -> Result<Vec<Ad>, Error> {
    //     Ok(Vec::new())
    // }
}
