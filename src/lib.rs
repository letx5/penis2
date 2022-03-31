/// A structure to create penises
pub struct PenisSpec {
    len: usize
}

impl PenisSpec {
    /// Creates a spec for a 3 long penis
    pub fn new() -> PenisSpec {
        PenisSpec::new_with_len(3)
    }
    /// Create an arbitatry penis spec
    pub fn new_with_len(x: usize) -> PenisSpec {
        PenisSpec {
            len: x
        }
    }
    /// Create a penis spec from the total string len.
    pub fn new_with_total_len(x: usize) -> Option<PenisSpec> {
        if x >= 2 {
            Some(PenisSpec {
                len: x - 2
            })
        } else {
            None
        }
    }
    /// Create a penis from spec
    pub fn generate(&self) -> String {
        format!("B{}D","=".repeat(self.len))
    }
    /// Print a penis to stdout.
    pub fn print(&self) {
        print!("{}", self.generate())
    }
    /// Print a penis to stdout with newline.
    pub fn println(&self) {
        println!("{}", self.generate())
    }
}

#[cfg(test)]
mod tests {
    use crate::PenisSpec;
    #[test]
    fn basic() {
        assert_eq!(PenisSpec::new().generate(), "B===D");
    }
    #[test]
    fn len() {
        assert_eq!(PenisSpec::new_with_len(4).generate(), "B====D");
        assert_eq!(PenisSpec::new_with_len(6).generate(), "B======D");
    }
    #[test]
    fn total_len() {
        assert_eq!(PenisSpec::new_with_total_len(6).unwrap().generate(), "B====D");
        assert_eq!(PenisSpec::new_with_total_len(8).unwrap().generate(), "B======D");
    }
}
