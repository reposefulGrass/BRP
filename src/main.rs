
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn documentation_example() {
        let line = "  \"  11 (BC_2, IO_AK14, output3, X, 10, 1, pull0),\" & -- PAD445";
        let scan_cell = ScanCell::parseLine(line);
    }
}