mod current_struct;
pub use current_struct::Current;

use crate::{cmdargs::CmdArgs, speak_trait::Speak, weather_trait::Weather};

impl Weather for Current {
    fn base_url_type() -> &'static str {
        "weather"
    }
}

impl Speak for Current {
    fn text(&self, _args: &CmdArgs) -> String {
        let weather = &self.weather[0].description;
        let degrees = &self.main.temp;
        format!("Currently there is {} degrees celsius and {}", degrees, weather)
    }
}