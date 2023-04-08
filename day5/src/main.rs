use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, opt, all_consuming},
    sequence::{delimited, preceded},
    IResult, Finish,
};

#[derive(Debug)]
struct Crate(char);

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag(" "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_cate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_hole(i)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

fn main() {
    let mut crate_lines = vec![];

    for line in include_str!("sample-input.txt").lines() {
        if let Ok((_rest, crate_line)) = all_consuming(parse_cate_line)(line).finish() {
            crate_lines.push(crate_line)
        }
    }

    for line in &crate_lines {
        println!("{line:?}")
    }
}
