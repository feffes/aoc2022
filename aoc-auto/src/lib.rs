use lazy_static::lazy_static;
use reqwest::blocking::Client;
use reqwest::cookie::Jar;
use reqwest::Error;
use reqwest::Url;
use std::env::var;
use std::time::Duration;

static USER_AGENT: &str = "github.com/feffes/aoc2022 by feffe@feffe.dev";
static YEAR: &str = "2022";

lazy_static! {
    static ref CLIENT: Client = {
        let jar = std::sync::Arc::new(Jar::default());
        jar.add_cookie_str(
            var("AOC_SESSION").unwrap().as_str(),
            &"https://adventofcode.com".parse::<Url>().unwrap());
        Client::builder()
        .timeout(Duration::from_secs(5))
        .cookie_provider(jar)
        .user_agent(USER_AGENT)
        .build()
        .unwrap()
        };
        // set this env var by curling or something
}

pub fn input(day: &str) -> Result<String, Error> {
    let text = CLIENT
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            YEAR, day
        ))
        .send()?
        .text()?;
    Ok(text)
}

#[cfg(test)]
mod test {
    #[test]
    fn input_test() {
        let day1 = std::fs::read_to_string("../01/input").expect("Couldn't read input file");
        assert_eq!(super::input("1").unwrap(), day1);
    }
}
