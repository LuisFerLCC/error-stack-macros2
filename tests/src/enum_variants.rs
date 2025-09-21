#[cfg(test)]
mod tests {
    use error_stack_macros2::Error;

    #[test]
    fn unit_variant_works() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display("unit variant")]
            UnitVariant,
        }

        assert_eq!(EnumType::UnitVariant.to_string(), "unit variant");
    }

    #[test]
    fn named_field_variant_works_without_interpolation() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display("named field variant")]
            NamedFieldVariant {
                _length: usize,
                _is_ascii: bool,
                _inner: String,
            },
        }

        let test_val = EnumType::NamedFieldVariant {
            _length: 5,
            _is_ascii: true,
            _inner: String::from("hello"),
        };
        assert_eq!(test_val.to_string(), "named field variant");
    }

    #[test]
    fn named_field_variant_works_with_interpolation_of_some_fields() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display("named field variant: {inner:?} has {length} characters")]
            NamedFieldVariant {
                length: usize,
                _is_ascii: bool,
                inner: String,
            },
        }

        let test_val = EnumType::NamedFieldVariant {
            length: 5,
            _is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            test_val.to_string(),
            "named field variant: \"hello\" has 5 characters"
        );
    }

    #[test]
    fn named_field_variant_works_with_interpolation_of_all_fields() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display(
                "named field variant: {inner:?} has {length} characters and is ascii={is_ascii}"
            )]
            NamedFieldVariant {
                length: usize,
                is_ascii: bool,
                inner: String,
            },
        }

        let test_val = EnumType::NamedFieldVariant {
            length: 5,
            is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            test_val.to_string(),
            "named field variant: \"hello\" has 5 characters and is ascii=true"
        );
    }

    #[test]
    fn tuple_variant_works_without_interpolation() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display("tuple variant")]
            TupleVariant(isize, isize, isize),
        }

        let test_val = EnumType::TupleVariant(5, 10, 15);
        assert_eq!(test_val.to_string(), "tuple variant");
    }

    #[test]
    fn tuple_variant_works_with_interpolation_of_some_fields() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display("tuple variant: point with y value {1}")]
            TupleVariant(isize, isize, isize),
        }

        let test_val = EnumType::TupleVariant(5, 10, 15);
        assert_eq!(
            test_val.to_string(),
            "tuple variant: point with y value 10"
        );
    }

    #[test]
    #[allow(dead_code)]
    fn tuple_variant_works_with_interpolation_of_all_fields() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display(
                "tuple variant: point {2} units in front of the origin, and with x and y coords ({0}, {1})"
            )]
            TupleVariant(isize, isize, isize),
        }

        let test_val = EnumType::TupleVariant(5, 10, 15);
        assert_eq!(
            test_val.to_string(),
            "tuple variant: point 15 units in front of the origin, and with x and y coords (5, 10)"
        );
    }
}
