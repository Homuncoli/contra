#[cfg(test)]
mod test {
    use lib_contra::{serialize::json::IntoJson, deserialize::{json::FromJson}};
    use proc_contra::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct EmptyStruct {
    }

    #[test]
    fn empty_struct_works() {
        let expected = EmptyStruct { };
        
        let json = IntoJson::to_json(&expected);
        assert!(json.is_ok());

        let result = FromJson::from_json(&json.unwrap());
        assert!(result.is_ok());

        assert_eq!(expected, result.unwrap());
    }
}
