#[macro_export]
macro_rules! test_serialize_fmt {
    ($name: ident, $x:expr, $y:expr) => {
        #[test]
        fn $name() {
            assert_eq!(serde_json::to_string($x).unwrap(), $y.replace(" ", ""));
        }
    };
}

#[macro_export]
macro_rules! own {
    ($x: expr) => {
        $x.to_string()
    };
}
