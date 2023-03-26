mod tests {
    use disp_array::DispArray;
    #[test]
    fn i32_test() {
        let numbers = [1, 2, 3, 4];
        assert_eq!(DispArray(numbers).to_string(), "[1, 2, 3, 4]");
    }

    #[test]
    fn f64_test() {
        let array = [1.1, 2.2, 3.3, 4.4];
        assert_eq!(DispArray(array).to_string(), "[1.1, 2.2, 3.3, 4.4]")
    }

    #[test]
    fn char_test() {
        let array = ['A', 'B', 'C', 'D'];
        assert_eq!(DispArray(array).to_string(), "[A, B, C, D]");
    }
}