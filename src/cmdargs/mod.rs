mod cmdargs_struct;
use crate::Result;
pub use cmdargs_struct::CmdArgs;

impl CmdArgs {
    pub fn get() -> Self {
        use structopt::StructOpt;
        Self::from_args()
    }
    pub fn set_logging(&self) -> Result<()> {
        Ok(env_logger::builder().filter_level(
            match self.verbose {
                0 => log::LevelFilter::Error,
                1 => log::LevelFilter::Info,
                2 => log::LevelFilter::Debug,
                v if v >= 3 => log::LevelFilter::Trace,
                _ => unreachable!(),
            }
        ).try_init()?)
    }
}