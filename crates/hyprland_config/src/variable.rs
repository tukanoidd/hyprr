use std::{
    fmt::{Display, Formatter},
    num::ParseIntError,
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{digit1, space1},
    combinator::verify,
    combinator::{map, map_res, opt, value},
    error::Error,
    multi::separated_list1,
    number::complete::float,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    AsChar, IResult,
};

use crate::util::{Float, Int, Parse};

pub struct Variable {
    pub name: String,
    pub value: VariableValue,
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${} = {}", self.name, self.value)
    }
}

impl Parse for Variable {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                Self::parse_name,
                space1,
                tag("="),
                space1,
                VariableValue::parse,
            )),
            |(name, _, _, _, value)| Self { name, value },
        )(input)
    }
}

impl Variable {
    fn parse_name(input: &str) -> IResult<&str, String> {
        map(
            preceded(
                tag("$"),
                verify(
                    take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                    |s: &str| s.chars().next().unwrap().is_alphabetic(),
                ),
            ),
            |str: &str| str.to_string(),
        )(input)
    }
}

macro_rules! variable_value {
    ($($variant:ident($ty:ty) $(doc: $doc:literal)*),* $(,)*) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum VariableValue {
            $(
                $(#[doc = $doc])*
                $variant($ty),
            )*
        }

        impl Display for VariableValue {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    match self {
                        $(
                            Self::$variant(val) => val.to_string(),
                        )*
                    }
                )
            }
        }
    };
}

variable_value!(
    Int(Int) doc: "true/false | yes/no | on/off | 0/1",
    Bool(Bool),
    Float(Float),
    Color(Color),
    Vec2(Vec2) doc: "0 0 | -10.9 99.1",
    Mod(Modifier),
    String(String),
    Gradient(Gradient),
    Variable(String),
);

impl Parse for VariableValue {
    fn parse(input: &str) -> IResult<&str, Self> {
        let string_literal = delimited(tag("\""), take_until("\""), tag("\""));

        alt((
            map(Variable::parse_name, Self::Variable),
            map(Gradient::parse, Self::Gradient),
            map(Color::parse, Self::Color),
            map(Vec2::parse, Self::Vec2),
            map(Int::parse, Self::Int),
            map(float, Self::Float),
            map(Bool::parse, Self::Bool),
            map(Modifier::parse, Self::Mod),
            map(string_literal, |s: &str| Self::String(s.to_string())),
        ))(input)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bool {
    True,
    False,

    Yes,
    No,

    On,
    Off,

    Zero,
    One,
}

impl Display for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Bool::True => "true",
                Bool::False => "false",

                Bool::Yes => "yes",
                Bool::No => "no",

                Bool::On => "on",
                Bool::Off => "off",

                Bool::One => "1",
                Bool::Zero => "0",
            }
        )
    }
}

impl Parse for Bool {
    fn parse(input: &str) -> IResult<&str, Self, Error<&str>> {
        alt((
            value(Bool::True, tag("true")),
            value(Bool::False, tag("false")),
            value(Bool::Yes, tag("yes")),
            value(Bool::No, tag("no")),
            value(Bool::On, tag("on")),
            value(Bool::Off, tag("off")),
            value(Bool::One, tag("1")),
            value(Bool::Zero, tag("0")),
        ))(input)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    /// rgba(b3ff1aee)
    RGBA(u64),
    /// rgb(b3ff1a)
    RGB(u64),
    /// 0xeeb3ff1a -> ARGB order
    Legacy(u64),
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::RGBA(rgba) => write!(f, "rgba({rgba:x})"),
            Color::RGB(rgb) => write!(f, "rgb({rgb:x})"),
            Color::Legacy(argb) => write!(f, "{argb:#x}"),
        }
    }
}

impl Parse for Color {
    fn parse(input: &str) -> IResult<&str, Self> {
        let rgba = delimited(
            tag("rgba("),
            map_res(take_until(")"), Color::rgba),
            tag(")"),
        );

        let rgb = delimited(tag("rgb("), map_res(take_until(")"), Color::rgb), tag(")"));

        let legacy = preceded(
            tag("0x"),
            map_res(take_while(|c: char| c.is_hex_digit()), Color::legacy),
        );

        alt((rgba, rgb, legacy))(input)
    }
}

impl Color {
    pub fn rgba(input: &str) -> Result<Self, ParseIntError> {
        u64::from_str_radix(input, 16).map(Color::RGBA)
    }

    pub fn rgb(input: &str) -> Result<Color, ParseIntError> {
        u64::from_str_radix(input, 16).map(Color::RGB)
    }

    pub fn legacy(input: &str) -> Result<Color, ParseIntError> {
        u64::from_str_radix(input, 16).map(Color::Legacy)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2(pub Float, pub Float);

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self(x, y) = self;

        write!(f, "{x} {y}")
    }
}

impl Parse for Vec2 {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_pair(float, tag(" "), float), |(x, y)| Vec2(x, y))(input)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Modifier {
    SHIFT,
    CAPS,
    CTRL(ControlModifier),
    ALT,
    MOD2,
    MOD3,
    SUPER(SuperModifier),
    MOD5,
}

impl Display for Modifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Modifier::CTRL(ctrl) => ctrl.to_string(),
                Modifier::SUPER(super_) => super_.to_string(),
                x => match x {
                    Modifier::SHIFT => "SHIFT",
                    Modifier::CAPS => "CAPS",
                    Modifier::ALT => "ALT",
                    Modifier::MOD2 => "MOD2",
                    Modifier::MOD3 => "MOD3",
                    Modifier::MOD5 => "MOD5",
                    _ => unreachable!(),
                }
                .to_string(),
            }
        )
    }
}

impl Parse for Modifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        let shift = value(Modifier::SHIFT, tag("SHIFT"));
        let caps = value(Modifier::CAPS, tag("CAPS"));
        let ctrl = map(ControlModifier::parse, Modifier::CTRL);
        let alt_ = value(Modifier::ALT, tag("ALT"));
        let mod2 = value(Modifier::MOD2, tag("MOD2"));
        let mod3 = value(Modifier::MOD3, tag("MOD3"));
        let super_ = map(SuperModifier::parse, Modifier::SUPER);
        let mod5 = value(Modifier::MOD5, tag("MOD5"));

        alt((shift, caps, ctrl, alt_, mod2, mod3, super_, mod5))(input)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ControlModifier {
    CTRL,
    CONTROL,
}

impl Display for ControlModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ControlModifier::CTRL => "CTRL",
                ControlModifier::CONTROL => "CONTROL",
            }
        )
    }
}

impl Parse for ControlModifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        let ctrl = value(ControlModifier::CTRL, tag("CTRL"));
        let control = value(ControlModifier::CONTROL, tag("CONTROL"));

        alt((ctrl, control))(input)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SuperModifier {
    SUPER,
    WIN,
    LOGO,
    MOD4,
}

impl Display for SuperModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SuperModifier::SUPER => "SUPER",
                SuperModifier::WIN => "WIN",
                SuperModifier::LOGO => "LOGO",
                SuperModifier::MOD4 => "MOD4",
            }
        )
    }
}

impl Parse for SuperModifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        let super_ = value(SuperModifier::SUPER, tag("SUPER"));
        let win = value(SuperModifier::WIN, tag("WIN"));
        let logo = value(SuperModifier::LOGO, tag("LOGO"));
        let mod4 = value(SuperModifier::MOD4, tag("MOD4"));

        alt((super_, win, logo, mod4))(input)
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Gradient {
    /// color1 color2 color3 ... (See [Color])
    pub colors: Vec<Color>,
    /// Deg - 20deg | 45deg | 345deg | ..
    pub angle: Option<u16>,
}

impl Display for Gradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.colors
                .iter()
                .map(ToString::to_string)
                .collect_vec()
                .join(" "),
            match self.angle {
                Some(angle) => format!(" {}deg", angle),
                None => String::new(),
            }
        )
    }
}

impl Parse for Gradient {
    fn parse(input: &str) -> IResult<&str, Self, Error<&str>> {
        let colors = verify(separated_list1(space1, Color::parse), |list: &[Color]| {
            list.len() > 1
        });
        let deg = opt(preceded(
            space1,
            terminated(map_res(digit1, |s: &str| s.parse()), tag("deg")),
        ));

        map(tuple((colors, deg)), |(colors, angle)| Self {
            colors,
            angle,
        })(input)
    }
}
