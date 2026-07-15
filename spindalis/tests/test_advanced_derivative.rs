#[cfg(test)]
mod tests {
    use spindalis::derivatives::advanced_derivative;
    use spindalis::polynomials::Polynomial;

    #[test]
    fn derivative() {
        let parsed = Polynomial::parse("3x + 2").unwrap();
        let result = advanced_derivative(&parsed, "x").unwrap();
        let expected = Polynomial::parse("3x").unwrap();
        //let expected = Polynomial::parse("3").unwrap();

        assert_eq!(result, expected);
    }
}
