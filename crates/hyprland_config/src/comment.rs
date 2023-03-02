use std::fmt::{Display, Formatter};

use nom::{
    branch::alt,
    bytes::{complete::tag, streaming::take_until},
    character::complete::not_line_ending,
    combinator::{map, map_res},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::util::Parse;

#[derive(Debug, Clone, PartialEq)]
pub enum Comment {
    OneLine(OneLineComment),
    MultiLine(MultiLineComment),
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OneLine(comment) => comment.to_string(),
                Self::MultiLine(comment) => comment.to_string(),
            }
        )
    }
}

impl Parse for Comment {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(MultiLineComment::parse, Comment::MultiLine),
            map(OneLineComment::parse, Comment::OneLine),
        ))(input)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OneLineComment(pub String);

impl Display for OneLineComment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parse for OneLineComment {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(preceded(tag("#"), not_line_ending), |s: &str| {
            Self(format!("#{s}"))
        })(input)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultiLineComment(pub String);

impl Display for MultiLineComment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parse for MultiLineComment {
    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(
            tuple((many1(tag("##")), take_until("##"), many1(tag("##")))),
            |(start, middle, end): (Vec<&str>, &str, Vec<&str>)| match start.len() == end.len() {
                true => Ok(Self(format!(
                    "{}{}{}",
                    start.join(""),
                    middle,
                    end.join("")
                ))),
                false => Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Tag,
                ))),
            },
        )(input)
    }
}
