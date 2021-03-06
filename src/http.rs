use crate::{Aoc, Level};
use html2md::parse_html;
use failure::{Error, bail};
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header;
use std::collections::HashMap;

const BASE: &str = "https://adventofcode.com";

fn get_url(aoc: &Aoc) -> Result<String, Error> {
    let url = match (aoc.day, aoc.year) {
        (Some(d), Some(y)) => format!("{}/{}/day/{}", BASE, y, d),
        _ => bail!("day or year not set"),
    };
    Ok(url)
}

fn get_content(aoc: &Aoc, suffix: &str) -> Result<String, Error> {
    let url = format!("{}{}", get_url(aoc)?, suffix);
    let cookie = format!("session={}", aoc.cookie);
    let input = Client::new()
        .get(&url)
        .header(header::COOKIE, cookie)
        .send()?
        .error_for_status()?
        .text()?;
    Ok(input)
}

pub fn get_brief(aoc: &Aoc) -> Result<(String, String), Error> {
    let brief = get_content(aoc, "")?;
    let title = get_title(&brief).unwrap_or_default();
    let brief = get_html_section(&brief, "main").unwrap_or_else(|| "".to_string());
    let brief = parse_html(&brief);
    let num_lines = brief.lines().count();
    let brief = brief.lines()
        .skip(2)
        .take(num_lines - 4)
        .map(|l| format!("{}\n", l))
        .collect::<String>()
        .trim()
        .to_string();
    Ok((title, brief))
}

pub fn get_input(aoc: &Aoc) -> Result<String, Error> {
    let input = get_content(aoc, "/input")?;
    Ok(input)
}

pub fn submit(aoc: &Aoc, solution: &str) -> Result<String, Error> {
    let url = format!("{}/answer", get_url(aoc)?);
    let cookie = format!("session={}", aoc.cookie);

    let level = match aoc.level {
        Level::First => 1,
        Level::Second => 2,
    };

    let mut params = HashMap::new();
    params.insert("level", level.to_string());
    params.insert("answer", solution.into());

    let resp = Client::new()
        .post(&url)
        .header(header::COOKIE, cookie)
        .form(&params)
        .send()?
        .error_for_status()?
        .text()?;

    let resp = get_html_section(&resp, "main").unwrap_or_else(|| "".to_string());
    let resp = parse_html(&resp);
    let resp = resp.trim().to_string();
    Ok(resp)
}

fn get_html_section(contents: &str, section: &str) -> Option<String> {
    let regex = format!("<{}>((.|\n)*?)</{}>", section, section);
    let regex = Regex::new(&regex).unwrap();
    let html = regex.captures(contents)?.get(1)?.as_str();
    Some(html.to_string())
}

pub fn verify(text: &str) -> bool {
    text.contains("That's the right answer!")
}

fn get_title(brief: &str) -> Option<String> {
    let regex = Regex::new("<h2>--- Day .*?: (.*?) ---</h2>").unwrap();
    let title = regex.captures(brief)?.get(1)?.as_str();
    Some(title.to_string())
}
