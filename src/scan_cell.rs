
use std::str::FromStr;
use nom::{Finish, error::Error};

use crate::parser::parse_cell;


#[allow(non_camel_case_types)]
#[derive(Default, Debug, PartialEq, Eq, EnumIter)]
pub enum BoundaryType {
    #[default] BC_0 = 0,
    BC_1,
    BC_2,
    BC_3,
    BC_4,
    BC_5,
    BC_6,
    BC_7,
    BC_8,
    BC_9,
    BC_10,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum ControlType {
    INTERNAL,
    #[default] OUTPUT3,
    CONTROL,
}
#[derive(Default, Debug, PartialEq, Eq)]
pub enum OutputDriverCondition {
    #[default] Z,
    PULL0,
}

// Structured by looking @ AC278: Microsemi BSDL Files Format
#[derive(Default, Debug, PartialEq, Eq)]
pub struct ScanCell {
    // the id of the cell
    pub id: u32,

    // the boundary cell type (IEEE spec says BC_0 through BC_10)
    pub boundary_type: BoundaryType,

    // the pin name
    pub port_id: String,

    // specifies the type of cell and what it does
    pub control_type: ControlType,

    // the value to be put into the scan cell flip-flop
    pub flip_flop_value: Option<u8>,

    // the id of the cell that can disable the output of this cell
    pub control_cell_id: u32,
    
    // the value that must be within cell "control_cell_id" to disable this cell
    pub control_cell_threshold: u32,

    // the condition of the output driver when this cell is disabled
    pub control_cell_condition: OutputDriverCondition,

    // any comment after -- at the end of the line (trimmed)
    pub comment: String,
}

impl FromStr for ScanCell {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_cell(s).finish() {
            Ok((_remaining, cell)) => Ok(cell),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scan_cell::*;

    #[test]
    fn documentation_example() {
        let line = "  \"  23 (BC_1, IO_AK01, output3, X, 15, 0, Z),\" & -- PAD762";
        let cell = ScanCell::from_str(line).unwrap();

        assert_eq!(
            cell,
            ScanCell {
                id: 23,
                boundary_type: BoundaryType::BC_1,
                port_id: String::from("IO_AK01"),
                control_type: ControlType::OUTPUT3,
                flip_flop_value: None,
                control_cell_id: 15,
                control_cell_threshold: 0,
                control_cell_condition: OutputDriverCondition::Z,
                comment: String::from("PAD762")
            }
        );
    }
}