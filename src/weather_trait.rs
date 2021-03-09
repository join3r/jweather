use crate::ipapi::IpApi;
use reqwest::{blocking, Url};
use serde::de;

use crate::{cmdargs::CmdArgs, Result};

pub trait Weather {
    fn get_request(args: &CmdArgs) -> Result<Self>
    where
        Self: Sized + de::DeserializeOwned,
    {
        let base_url = format!(
            "http://api.openweathermap.org/data/2.5/{}",
            Self::base_url_type()
        );
        let url = get_url(&args, &base_url)?;
        log::info!("calling url: {:?}", &url);
        let resp = blocking::get(url)?.error_for_status()?.text()?;
        log::trace!("Response: {}", &resp);
        Ok(serde_json::from_str(&resp)?)
    }
    fn base_url_type() -> &'static str;
}

fn get_url(args: &CmdArgs, base_url: &str) -> Result<Url> {
    let url = base_url;
    let mut params = Vec::new();
    match (args.city.is_some(), args.lat.is_some()) {
        // city provided by user
        (true, false) => params.push(("q", args.city.clone().unwrap())), // unwrap: sure to be some
        // location proivded by user
        (false, true) => {
            params.push(("lon", args.lon.unwrap().to_string()));
            params.push(("lat", args.lat.unwrap().to_string()));
        }
        // nothing provided by user
        (false, false) => {
            let ipapi = IpApi::get()?;
            println!("City/Location not provided.");
            println!(
                "Using {}, (lon={}, lat={})",
                ipapi.city, ipapi.lon, ipapi.lat
            );
            println!("Check --help for available parameters");
            params.push(("lon", ipapi.lon.to_string()));
            params.push(("lat", ipapi.lat.to_string()));
        }
        _ => unreachable!(),
    };
    params.push(("units", "metric".into()));
    params.push(("lang", "en".into()));
    let default_api_key = include_str!("api-key.secret").to_string();
    let appid = args.api_key.clone().unwrap_or(default_api_key);
    params.push(("appid", appid));
    log::debug!("Creating url from: {}\n{:?}", url, params);
    Ok(Url::parse_with_params(&url, &params)?)
}
