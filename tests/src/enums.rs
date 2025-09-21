#[cfg(test)]
mod tests {
    use error_stack_macros2::Error;

    #[test]
    fn empty_enum_works_without_display_attr() {
        #[derive(Debug, Error)]
        enum _EmptyEnumType {}
    }

    #[test]
    fn empty_enum_works_with_display_attr() {
        #[derive(Debug, Error)]
        #[display("this display attr is unnecessary")]
        enum _EmptyEnumType {}
    }

    #[test]
    #[allow(dead_code)]
    #[allow(clippy::enum_variant_names)]
    fn enum_works_with_display_attr_default() {
        #[derive(Debug, Error)]
        #[display("enum type")]
        enum EnumType {
            UnitVariant,

            NamedFieldVariant {
                _length: usize,
                _is_ascii: bool,
                _inner: String,
            },

            TupleVariant(isize, isize, isize),
        }

        let unit = EnumType::UnitVariant;
        assert_eq!(unit.to_string(), "enum type");

        let named_field = EnumType::NamedFieldVariant {
            _length: 5,
            _is_ascii: true,
            _inner: String::from("hello"),
        };
        assert_eq!(named_field.to_string(), "enum type");

        let tuple = EnumType::TupleVariant(5, 10, 15);
        assert_eq!(tuple.to_string(), "enum type");
    }

    #[test]
    #[allow(clippy::enum_variant_names)]
    fn enum_works_with_display_attr_default_and_some_variants() {
        #[derive(Debug, Error)]
        #[display("enum type")]
        enum EnumType {
            UnitVariant,

            #[display(
                "named field variant: {inner:?} has {length} characters and is ascii={is_ascii}"
            )]
            NamedFieldVariant {
                length: usize,
                is_ascii: bool,
                inner: String,
            },

            #[display(
                "tuple variant: point {2} units in front of the origin, and with x and y coords ({0}, {1})"
            )]
            TupleVariant(isize, isize, isize),
        }

        let unit = EnumType::UnitVariant;
        assert_eq!(unit.to_string(), "enum type");

        let named_field = EnumType::NamedFieldVariant {
            length: 5,
            is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            named_field.to_string(),
            "named field variant: \"hello\" has 5 characters and is ascii=true"
        );

        let tuple = EnumType::TupleVariant(5, 10, 15);
        assert_eq!(
            tuple.to_string(),
            "tuple variant: point 15 units in front of the origin, and with x and y coords (5, 10)"
        );
    }

    #[test]
    #[allow(clippy::enum_variant_names)]
    fn enum_works_with_display_attr_default_and_all_variants() {
        #[derive(Debug, Error)]
        #[display("enum type")]
        enum EnumType {
            #[display("unit variant")]
            UnitVariant,

            #[display(
                "named field variant: {inner:?} has {length} characters and is ascii={is_ascii}"
            )]
            NamedFieldVariant {
                length: usize,
                is_ascii: bool,
                inner: String,
            },

            #[display(
                "tuple variant: point {2} units in front of the origin, and with x and y coords ({0}, {1})"
            )]
            TupleVariant(isize, isize, isize),
        }

        let unit = EnumType::UnitVariant;
        assert_eq!(unit.to_string(), "unit variant");

        let named_field = EnumType::NamedFieldVariant {
            length: 5,
            is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            named_field.to_string(),
            "named field variant: \"hello\" has 5 characters and is ascii=true"
        );

        let tuple = EnumType::TupleVariant(5, 10, 15);
        assert_eq!(
            tuple.to_string(),
            "tuple variant: point 15 units in front of the origin, and with x and y coords (5, 10)"
        );
    }

    #[test]
    #[allow(clippy::enum_variant_names)]
    fn enum_works_with_display_attr_all_variants() {
        #[derive(Debug, Error)]
        enum EnumType {
            #[display("unit variant")]
            UnitVariant,

            #[display(
                "named field variant: {inner:?} has {length} characters and is ascii={is_ascii}"
            )]
            NamedFieldVariant {
                length: usize,
                is_ascii: bool,
                inner: String,
            },

            #[display(
                "tuple variant: point {2} units in front of the origin, and with x and y coords ({0}, {1})"
            )]
            TupleVariant(isize, isize, isize),
        }

        let unit = EnumType::UnitVariant;
        assert_eq!(unit.to_string(), "unit variant");

        let named_field = EnumType::NamedFieldVariant {
            length: 5,
            is_ascii: true,
            inner: String::from("hello"),
        };
        assert_eq!(
            named_field.to_string(),
            "named field variant: \"hello\" has 5 characters and is ascii=true"
        );

        let tuple = EnumType::TupleVariant(5, 10, 15);
        assert_eq!(
            tuple.to_string(),
            "tuple variant: point 15 units in front of the origin, and with x and y coords (5, 10)"
        );
    }
}
