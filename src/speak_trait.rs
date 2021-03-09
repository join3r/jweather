use crate::{cmdargs::CmdArgs, Result};
use reqwest::{blocking, Url};
use std::io::Cursor;

pub trait Speak {
    fn text(&self, args: &CmdArgs) -> String;
    fn speak(&self, args: &CmdArgs) -> Result<()> {
        let text = self.text(args);
        let language = "en";
        let base = format!("https://translate.google.com/translate_tts?ie=UTF-8&tl={}&total=1&idx=0&tl={}&client=tw-ob", language, language);
        let mut params = Vec::new();
        params.push(("q", text.clone()));
        params.push(("textlen", text.len().to_string()));
        let url = Url::parse_with_params(&base, &params)?;
        let resp = blocking::get(url)?.error_for_status()?.bytes()?.to_vec();
        rodio_talk(resp)?;
        Ok(())
    }
}

fn rodio_talk(data: Vec<u8>) -> Result<()> {
    let (_stream, handle) = rodio::OutputStream::try_default()?;
    let sink = rodio::Sink::try_new(&handle)?;
    sink.append(rodio::Decoder::new(Cursor::new(data))?);
    sink.sleep_until_end();
    Ok(())
}
