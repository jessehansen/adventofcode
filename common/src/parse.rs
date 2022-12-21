// parsing helpers
use anyhow::*;
use std::str::pattern::Pattern;

pub fn trim(contents: &str) -> String {
    contents.trim().to_string()
}

pub fn wrap_parse_error<T, TErr>(result: std::result::Result<T, TErr>) -> Result<T>
where
    TErr: std::fmt::Display,
{
    match result {
        std::result::Result::Ok(value) => Ok(value),
        std::result::Result::Err(err) => Err(anyhow!("parse error {}", err)),
    }
}

pub fn parse_all<T>(contents: &str) -> Result<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    wrap_parse_error(contents.trim().parse())
}

pub fn parse_untrimmed<T>(contents: &str) -> Result<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    wrap_parse_error(contents.parse())
}

pub fn parse_lines<T>(contents: &str) -> Result<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    contents
        .lines()
        .map(|x| wrap_parse_error(x.parse()))
        .collect()
}

pub fn parse_split<T, P>(input: &str, separator: P) -> Result<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
    P: for<'a> Pattern<'a>,
{
    input
        .split(separator)
        .map(|x| wrap_parse_error(x.parse()))
        .collect()
}

pub fn parse_chars<T>(contents: &str) -> Result<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    contents
        .trim()
        .chars()
        .map(|x| wrap_parse_error(x.to_string().parse()))
        .collect()
}

pub fn parse_line_groups<T>(contents: &str) -> Result<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    contents
        .split("\n\n")
        .map(|x| wrap_parse_error(x.parse()))
        .collect()
}

pub fn parse_line_pairs<T>(contents: &str, separator: &str) -> Result<Vec<(T, T)>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    contents
        .lines()
        .map(|x| wrap_parse_error(parse_pair(x, separator)))
        .collect()
}

pub fn parse_pair<'a, T0, T1, P>(contents: &'a str, separator: P) -> Result<(T0, T1)>
where
    T0: std::str::FromStr,
    <T0 as std::str::FromStr>::Err: std::fmt::Display,
    T1: std::str::FromStr,
    <T1 as std::str::FromStr>::Err: std::fmt::Display,
    P: Pattern<'a>,
{
    let mut parts = contents.split(separator);
    Ok((
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed pair"))?
                .parse(),
        )?,
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed pair"))?
                .parse(),
        )?,
    ))
}

pub fn parse_pair_by<T0, FParse0, T1, FParse1>(
    contents: &str,
    separator: &str,
    parse0: FParse0,
    parse1: FParse1,
) -> Result<(T0, T1)>
where
    FParse0: Fn(&str) -> Result<T0>,
    FParse1: Fn(&str) -> Result<T1>,
{
    let mut parts = contents.split(separator);
    Ok((
        parse0(parts.next().ok_or_else(|| anyhow!("malformed pair"))?)?,
        parse1(parts.next().ok_or_else(|| anyhow!("malformed pair"))?)?,
    ))
}

pub fn parse_triple<T0, T1, T2>(contents: &str, separator: &str) -> Result<(T0, T1, T2)>
where
    T0: std::str::FromStr,
    <T0 as std::str::FromStr>::Err: std::fmt::Display,
    T1: std::str::FromStr,
    <T1 as std::str::FromStr>::Err: std::fmt::Display,
    T2: std::str::FromStr,
    <T2 as std::str::FromStr>::Err: std::fmt::Display,
{
    let mut parts = contents.split(separator);
    Ok((
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed triple"))?
                .parse(),
        )?,
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed triple"))?
                .parse(),
        )?,
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed triple"))?
                .parse(),
        )?,
    ))
}

// grabs the 2 items at ix0 and ix1, in a string separated by separator
pub fn grab_2<'a, T0, T1, P: Pattern<'a>>(
    contents: &'a str,
    separator: P,
    ix0: usize,
    ix1: usize,
) -> Result<(T0, T1)>
where
    T0: std::str::FromStr,
    <T0 as std::str::FromStr>::Err: std::fmt::Display,
    T1: std::str::FromStr,
    <T1 as std::str::FromStr>::Err: std::fmt::Display,
{
    let mut parts = contents.split(separator);
    let mut ix = 0;
    while ix < ix0 {
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix0))?;

        ix += 1;
    }
    let first = wrap_parse_error(
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix0))?
            .parse(),
    )?;
    ix += 1;
    while ix < ix1 {
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix1))?;

        ix += 1;
    }

    Ok((
        first,
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix1))?
                .parse(),
        )?,
    ))
}

// grabs the 3 items at ix0, ix1, and ix2, in a string separated by separator
pub fn grab_3<T0, T1, T2>(
    contents: &str,
    separator: &str,
    ix0: usize,
    ix1: usize,
    ix2: usize,
) -> Result<(T0, T1, T2)>
where
    T0: std::str::FromStr,
    <T0 as std::str::FromStr>::Err: std::fmt::Display,
    T1: std::str::FromStr,
    <T1 as std::str::FromStr>::Err: std::fmt::Display,
    T2: std::str::FromStr,
    <T2 as std::str::FromStr>::Err: std::fmt::Display,
{
    let mut parts = contents.split(separator);
    let mut ix = 0;
    while ix < ix0 {
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix0))?;

        ix += 1;
    }
    let first = wrap_parse_error(
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix0))?
            .parse(),
    )?;
    ix += 1;
    while ix < ix1 {
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix1))?;

        ix += 1;
    }
    let second = wrap_parse_error(
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix1))?
            .parse(),
    )?;
    ix += 1;

    while ix < ix2 {
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix2))?;

        ix += 1;
    }

    Ok((
        first,
        second,
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix2))?
                .parse(),
        )?,
    ))
}

pub trait Substring {
    fn substring(&self, start_index: usize, end_index: usize) -> &str;
}

impl Substring for str {
    fn substring(&self, start_index: usize, end_index: usize) -> &str {
        let (_, rest) = self.split_at(start_index);

        let (substr, _) = rest.split_at(end_index);

        substr
    }
}
