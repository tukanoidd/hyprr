use nom::{bytes::complete::take_till, combinator::map_res, IResult};

pub type Int = i32;
pub type Float = f32;

pub trait Parse {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl Parse for Int {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        map_res(take_till(|c: char| c.is_whitespace()), |str: &str| {
            str.parse::<Int>()
        })(input)
    }
}
