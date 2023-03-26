use std::fmt::{Display, Formatter, Result};

// https://rsdlt.github.io/posts/rust-use-newtype-pattern-display-trait-array-generics/
pub struct DispArray<T: Display, const N: usize>([T; N]);
impl<T: Display, const N: usize> Display for DispArray<T, N> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let separator = ", ";
        let mut string = String::from('[');
        let last = self.0.last().unwrap();
        for element in &self.0 {
            string.push_str(&element.to_string());
            if !std::ptr::eq(element, last) {
                string.push_str(&separator);
            }
        }
        string.push(']');
        write!(f, "{string}")
    }
}