fn compute(a: u32, b: u32) -> u32 {
    // To ensure the result type is always the same as the input types, we can use the `u32` type
    a + b * 4u32
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
