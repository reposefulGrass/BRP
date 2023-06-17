
use nom::{
    IResult, 
    bytes::complete::{take_till, take_while1, tag},
    sequence::{tuple, preceded}, 
    combinator::map_res, 
    character::complete::digit1,
    error
};

use strum::IntoEnumIterator;
use crate::scan_cell::*;


fn parse_cell_id (s: &str) -> IResult<&str, u32> {
    map_res(digit1, |s| u32::from_str_radix(s, 10))(s)
}

fn parse_boundary_type (s: &str) -> IResult<&str, BoundaryType> {
    map_res(
        preceded(tag("BC_"), digit1),
        |d| BoundaryType::iter().nth(usize::from_str_radix(d, 10).unwrap()).ok_or("Invalid Boundary Type")
    )(s)
}

fn parse_port_id (s: &str) -> IResult<&str, String> {
    map_res(
        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
        |s: &str| Ok::<String, error::Error<&str>>(s.to_owned())
    )(s)
}

pub fn parse_cell(input: &str) -> IResult<&str, ScanCell>  {
    let until_digit = take_till(|c: char| c.is_digit(10));
    let space = take_while1(|c: char| c.is_whitespace());
    let comma_sep = take_while1(|c: char| c == ' ' || c == ',');

    let (input, (_, id, _)) = tuple((until_digit, parse_cell_id, space))(input)?;
    let (input, (_, bt)) = tuple((tag("("), parse_boundary_type))(input)?;
    let (input, (_, port_id)) = tuple((comma_sep, parse_port_id))(input)?;

    Ok((input, ScanCell {
        id: id,
        boundary_type: bt,
        port_id: port_id,
        ..Default::default()
    }))
}