mod forecast_struct;
pub use forecast_struct::Forecast;

use crate::{cmdargs::CmdArgs, speak_trait::Speak, weather_trait::Weather};

impl Weather for Forecast {
    fn base_url_type() -> &'static str {
        "forecast"
    }
}

impl Speak for Forecast {
    fn text(&self, _args: &CmdArgs) -> String {
        todo!()
    }
}