use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::space0;
use nom::combinator::{map, map_res};
use nom::sequence::{delimited, tuple};
use nom::IResult;
use smart_default::SmartDefault;

use crate::{
    util::{Float, Int, Parse},
    variable::{Bool, Color, Gradient},
};

pub struct Section {
    pub name: String,
    pub ty: SectionType,
}

impl Parse for Section {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let mut general = map(
            tuple((
                tag("general"),
                space0,
                delimited(
                    tag("{"),
                    map_res(take_until("}"), |s: &str| {
                        GeneralData::parse(s).map(|(rest, data)| {
                            (
                                rest,
                                Section {
                                    name: "general".to_string(),
                                    ty: SectionType::General(data),
                                },
                            )
                        })
                    }),
                    tag("}"),
                ),
            )),
            |(_, _, (_, s))| s,
        );

        // Todo: more

        general(input)
    }
}

pub enum SectionType {
    General(GeneralData),
    Decoration,
    Animation,
    Input,
    Touchpad,
    TouchDevice,
    Gestures,
    Misc,
    Binds,
    /// Only for developers
    Debug,
}

#[derive(Debug, Clone, PartialEq, SmartDefault)]
pub struct GeneralData {
    #[default(1.0)]
    pub sensitivity: Float,
    #[default(1)]
    pub border_size: Int,
    #[default(Bool::True)]
    pub no_border_radius: Bool,
    #[default(5)]
    pub gaps_in: Int,
    #[default(20)]
    pub gaps_out: Int,
    #[default(Gradient { colors: vec![Color::Legacy(0xffffffff)], angle: None })]
    pub col_inactive_border: Gradient,
    #[default(Gradient { colors: vec![Color::Legacy(0xff444444)], angle: None })]
    pub col_active_border: Gradient,
    #[default(Gradient { colors: vec![Color::Legacy(0x66777700)], angle: None })]
    pub col_group_border: Gradient,
    #[default(Gradient { colors: vec![Color::Legacy(0x66ffff00)], angle: None })]
    pub col_border_active: Gradient,
    pub cursor_inactive_timeout: Int,
    #[default(Layout::Dwindle)]
    pub layout: Layout,
    #[default(Bool::False)]
    pub no_cursor_warps: Bool,
    #[default(Bool::False)]
    pub apply_sens_to_raw: Bool,
    #[default(Bool::False)]
    pub resize_on_border: Bool,
    #[default(15)]
    pub extend_border_grab_area: Int,
    #[default(Bool::True)]
    pub hover_icon_on_border: Bool,
}

impl Parse for GeneralData {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Layout {
    Master,
    Dwindle,
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Layout::Master => "master",
                Layout::Dwindle => "dwindle",
            }
        )
    }
}

impl Parse for Layout {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let master = map(tag("master"), |_| Layout::Master);
        let dwindle = map(tag("dwindle"), |_| Layout::Dwindle);

        alt((master, dwindle))(input)
    }
}
