use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Forecast {
    pub cod: String,
    pub message: f64,
    pub cnt: f64,
    pub list: Vec<List>,
    pub city: City,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct List {
    pub dt: f64,
    pub main: Main,
    pub weather: Vec<Weather>,
    pub clouds: Clouds,
    pub wind: Wind,
    pub visibility: f64,
    pub pop: f64,
    pub sys: Sys,
    pub dt_txt: String,
    pub snow: Option<Snow>,
    pub rain: Option<Rain>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Main {
    pub temp: f64,
    #[serde(rename = "feels_like")]
    pub feels_like: f64,
    #[serde(rename = "temp_min")]
    pub temp_min: f64,
    #[serde(rename = "temp_max")]
    pub temp_max: f64,
    pub pressure: f64,
    #[serde(rename = "sea_level")]
    pub sea_level: f64,
    #[serde(rename = "grnd_level")]
    pub grnd_level: f64,
    pub humidity: f64,
    #[serde(rename = "temp_kf")]
    pub temp_kf: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Weather {
    pub id: f64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Clouds {
    pub all: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Sys {
    pub pod: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Snow {
    #[serde(rename = "3h")]
    pub n3_h: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Rain {
    #[serde(rename = "3h")]
    pub n3_h: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct City {
    pub id: f64,
    pub name: String,
    pub coord: Coord,
    pub country: String,
    pub population: f64,
    pub timezone: f64,
    pub sunrise: f64,
    pub sunset: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}
