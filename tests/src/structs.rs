#[cfg(test)]
mod tests {
    use error_stack_macros2::Error;

    #[test]
    fn unit_struct_works() {
        #[derive(Debug, Error)]
        #[display("unit struct")]
        struct UnitStructType;

        assert_eq!(UnitStructType.to_string(), "unit struct");
    }

    #[test]
    fn named_field_struct_works_without_interpolation() {
        #[derive(Debug, Error)]
        #[display("named field struct")]
        struct NamedFieldStructType {
            _length: usize,
            _is_ascii: bool,
            _inner: String,
        }

        let test_val = NamedFieldStructType {
            _length: 5,
            _is_ascii: true,
            _inner: String::from("hello"),
        };
        assert_eq!(test_val.to_string(), "named field struct");
    }

    #[test]
    fn named_field_struct_works_with_interpolation_of_some_fields() {
        #[derive(Debug, Error)]
        #[display("named field struct: {inner:?} has {length} characters")]
        struct NamedFieldStructType {
            length: usize,
            _is_ascii: bool,
            inner: String,
        }

        let test_val = NamedFieldStructType {
            length: 5,
            _is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            test_val.to_string(),
            "named field struct: \"hello\" has 5 characters"
        );
    }

    #[test]
    fn named_field_struct_works_with_interpolation_of_all_fields() {
        #[derive(Debug, Error)]
        #[display(
            "named field struct: {inner:?} has {length} characters and is ascii={is_ascii}"
        )]
        struct NamedFieldStructType {
            length: usize,
            is_ascii: bool,
            inner: String,
        }

        let test_val = NamedFieldStructType {
            length: 5,
            is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            test_val.to_string(),
            "named field struct: \"hello\" has 5 characters and is ascii=true"
        );
    }

    #[test]
    #[allow(dead_code)]
    fn tuple_struct_works_without_interpolation() {
        #[derive(Debug, Error)]
        #[display("tuple struct")]
        struct TupleStructType(isize, isize, isize);

        let test_val = TupleStructType(5, 10, 15);
        assert_eq!(test_val.to_string(), "tuple struct");
    }

    #[test]
    #[allow(dead_code)]
    fn tuple_struct_works_with_interpolation_of_some_fields() {
        #[derive(Debug, Error)]
        #[display("tuple struct: point with y value {1}")]
        struct TupleStructType(isize, isize, isize);

        let test_val = TupleStructType(5, 10, 15);
        assert_eq!(test_val.to_string(), "tuple struct: point with y value 10");
    }

    #[test]
    #[allow(dead_code)]
    fn tuple_struct_works_with_interpolation_of_all_fields() {
        #[derive(Debug, Error)]
        #[display(
            "tuple struct: point {2} units in front of the origin, and with x and y coords ({0}, {1})"
        )]
        struct TupleStructType(isize, isize, isize);

        let test_val = TupleStructType(5, 10, 15);
        assert_eq!(
            test_val.to_string(),
            "tuple struct: point 15 units in front of the origin, and with x and y coords (5, 10)"
        );
    }
}
