use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CmdArgs {
    #[structopt(short,long,help="API KEY for openweathermap.org")]
    pub api_key: Option<String>,
    #[structopt(long,help="City", conflicts_with_all=&["lon", "lat"])]
    pub city: Option<String>,
    #[structopt(long,help="longitude", requires="lat", conflicts_with="city")]
    pub lon: Option<i64>,
    #[structopt(long,help="latitude", requires="lon")] 
    pub lat: Option<i64>,
    #[structopt(short,long,help="Current weather (default)",conflicts_with="forecast")]
    pub current: bool,
    #[structopt(short,long,help="Forecast weather (todo)",conflicts_with="current")]
    pub forecast: bool,
    #[structopt(short,help="Show verbose messages",takes_value=false,multiple=true,parse(from_occurrences))]
    pub verbose: u8,
}