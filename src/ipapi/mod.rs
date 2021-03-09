mod ipapi_struct;
pub use ipapi_struct::IpApi;
use reqwest::blocking;

use crate::Result;

impl IpApi {
    pub fn get() -> Result<Self> {
        let url = "http://ip-api.com/json/";
        let req = blocking::get(url)?.error_for_status()?.text()?;
        Ok(serde_json::from_str(&req)?)
    }
}