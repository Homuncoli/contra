#[cfg(test)]
mod test {
    use lib_contra::{serialize::json::IntoJson, deserialize::{json::FromJson}};
    use proc_contra::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct EmptyStruct {}

    #[test]
    fn empty_struct_works() {
        let expected = EmptyStruct { };
        
        let json = IntoJson::to_json(&expected);
        assert!(json.is_ok());

        let result = FromJson::from_json(&json.unwrap());
        assert!(result.is_ok());

        assert_eq!(expected, result.unwrap());
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct PrimitiveDataTypesStruct {
        i8:   i32,
        i16:  i16,
        i32:  i32,
        i64:  i64,
        i128: i128,
        u8:   u32,
        u16:  u16,
        u32:  u32,
        u64:  u64,
        u128: u128,
        usize: usize,
        isize: isize,
        string: String,
    }

    impl PrimitiveDataTypesStruct {
        fn new() -> Self {
            PrimitiveDataTypesStruct {
                i8:     1000,
                i16:    i16::MAX,
                i32:    i32::MAX,
                i64:    i64::MAX,
                i128:   i128::MAX,
                u8:     1000,
                u16:    u16::MAX,
                u32:    u32::MAX,
                u64:    u64::MAX,
                u128:   u128::MAX,
                usize:  usize::MAX,
                isize:  isize::MAX,
                string: "Hello World!".to_string(),
            }
        }
    }

    #[test]
    fn primitive_data_struct_works() {
        let expected = PrimitiveDataTypesStruct::new();
        
        let json = IntoJson::to_json(&expected);
        assert!(json.is_ok());

        let result = FromJson::from_json(&json.unwrap());
        assert!(result.is_ok());

        assert_eq!(expected, result.unwrap());
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct NestedDataStruct {
        p1: PrimitiveDataTypesStruct,
        p2: PrimitiveDataTypesStruct,
    }

    #[test]
    fn nested_data_struct_works() {
        let expected = NestedDataStruct {
            p1: PrimitiveDataTypesStruct::new(),
            p2: PrimitiveDataTypesStruct::new(),
        };
        
        let json = IntoJson::to_json(&expected);
        assert!(json.is_ok());
        let json = json.unwrap();

        let result = FromJson::from_json(&json);
        assert!(result.is_ok());
        let result = result.unwrap();

        assert_eq!(expected, result);
    }
}
