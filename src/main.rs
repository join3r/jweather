mod ipapi;
mod forecast;
use forecast::Forecast;
mod current;
use current::Current;
mod cmdargs;
use cmdargs::CmdArgs;
mod weather_trait;
mod speak_trait;

pub use anyhow::Result;
use speak_trait::Speak;
use weather_trait::Weather;

fn main() -> Result<()> {
    let args = CmdArgs::get();
    args.set_logging()?;
    if args.forecast {
        println!("Forecast: ");
        let forecast = Forecast::get_request(&args)?;
        forecast.speak(&args)?;
    } else {
        println!("Current weather: ");
        let current = Current::get_request(&args)?;
        current.speak(&args)?;
    }
    Ok(())
}
