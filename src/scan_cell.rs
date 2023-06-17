
enum BoundaryType {
    BC_0,
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

enum ControlType {
    INTERNAL,
    OUTPUT3,
    CONTROL,
}

enum OutputDriverCondition {
    Z,
    PULL0,
}

// Structured by looking @ AC278: Microsemi BSDL Files Format
struct ScanCell {
    // the id of the cell
    id: u32,

    // the boundary cell type (IEEE spec says BC_0 through BC_10)
    boundary_type: BoundaryType,

    // the pin name
    port_id: String,

    // specifies the type of cell and what it does
    control_type: ControlType,

    // the value to be put into the scan cell flip-flop
    flip_flop_value: Option<u8>,

    // the id of the cell that can disable the output of this cell
    control_cell_id: u32,
    
    // the value that must be within cell "control_cell_id" to disable this cell
    control_cell_threshold: u8,

    // the condition of the output driver when this cell is disabled
    control_cell_condition: OutputDriverCondition,

    // any comment after -- at the end of the line
    comment: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn documentation_example() {
        let line = "  \"  11 (BC_2, IO_AK14, output3, X, 10, 1, pull0),\" & -- PAD445";
        let scan_cell = ScanCell::parseLine(line);
    }
}