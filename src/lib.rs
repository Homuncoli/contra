#[cfg(test)]
mod test {
    use lib_contra::serialize::json::IntoJson;
    use proc_contra::Serialize;

    #[derive(Serialize)]
    struct EmptyStruct {}

    #[test]
    fn empty_struct_works() {
        let obj = EmptyStruct {};
        
        let result = IntoJson::to_json(&obj);

        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }
}
