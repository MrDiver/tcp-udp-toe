struct TicField {
    bits: [u8; 3],
}

impl TicField {
    fn new() -> TicField {
        TicField { bits: [0, 0, 0] }
    }
    fn is_field_set(n: u8) -> bool {
        // Hier mach mal wie du gesagt hattest mit bit operationen
        true
    }
}
