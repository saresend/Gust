#[macro_export]
macro_rules! test_serialize_fmt {
    ($x:expr, $y:expr) => {
        #[test]
        fn test() {
            assert_eq!(serde_json::to_string($x).unwrap(), $y);
        }
    };
}
