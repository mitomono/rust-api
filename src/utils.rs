pub mod response {
    #![allow(non_snake_case)]

    use utoipa::ToSchema;

    use crate::books::Books;
    use crate::members::Members;

    #[derive(ToSchema)]
    pub struct MembersResponse {
        pub Ok: Vec<Members>,
    }
    #[derive(ToSchema)]
    pub struct BooksResponse {
        pub Ok: Vec<Books>,
    }
    #[derive(ToSchema)]
    pub struct MemberResponse {
        pub Ok: Members,
    }
    #[derive(ToSchema)]
    pub struct BookResponse {
        pub Ok: Books,
    }
    #[derive(ToSchema)]
    pub struct DeleteResponse {
        pub deleted: usize,
    }
    #[derive(ToSchema)]
    pub struct ErrorResponse {
        pub Err: String,
    }
}

pub mod check {
    use std::collections::HashMap;

    use crate::error_handler::CustomError;

    /// Check if a &str is a int number.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::validate_int("2") {
    ///     Ok(n) => assert_eq!(2, n),
    ///     Err(e) => panic!("Returned Err! => {e}"),
    /// }
    /// ```
    /// ```
    /// use lib_api::utils::check;
    ///
    /// match check::validate_int("a") {
    ///     Err(e) if e.to_string() == "Error parsing string: 'a', not a valid integer" => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    ///     Ok(_) => panic!("Returned an Ok variant!"),
    /// }
    ///```
    pub fn validate_int(int_str: &str) -> Result<i32, CustomError> {
        int_str.parse::<i32>().map_err(|_| {
            CustomError::new(
                400,
                format!("Error parsing string: '{int_str}', not a valid integer"),
            )
        })
    }

    /// Check if a &str is a float number.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::validate_float("1.1") {
    ///     Ok(n) => assert_eq!(1.1, n),
    ///     Err(e) => panic!("Returned Err! => {e}"),
    /// }
    /// ```
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::validate_float("a") {
    ///     Err(e) if e.to_string() == "Error parsing string: 'a', not a valid float" => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    ///     Ok(_) => panic!("Returned an Ok variant!"),
    /// }
    ///```
    pub fn validate_float(float_str: &str) -> Result<f64, CustomError> {
        float_str.parse::<f64>().map_err(|_| {
            CustomError::new(
                400,
                format!("Error parsing string: '{float_str}', not a valid float"),
            )
        })
    }

    /// Check if a all items of &str comma separated items its a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::parse_ids("1,2") {
    ///     Ok(vec_n) => assert_eq!(vec![1, 2], vec_n),
    ///     Err(e) => panic!("Returned Err! => {e}"),
    /// }
    /// ```
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::parse_ids("a,2") {
    ///     Err(e) if e.to_string() == "Error parsing string: 'a', not a valid integer" => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    ///     Ok(_) => panic!("Returned an Ok variant!"),
    /// }
    /// ```
    pub fn parse_ids(ids_str: &str) -> Result<Vec<i32>, CustomError> {
        let ids: Vec<i32> = ids_str
            .split(',')
            .map(validate_int)
            .collect::<Result<Vec<i32>, CustomError>>()?;
        Ok(ids)
    }

    /// Check if a params for member are correct.
    ///
    /// pub struct Member {
    ///     pub id: i32,
    ///     pub first_name: String,
    ///     pub last_name: String,
    ///     pub email: String,
    ///     pub address: String,
    ///     pub age: i32,
    /// }
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_api::utils::check;
    /// use std::collections::HashMap;
    ///
    /// let mut params = HashMap::new();
    /// params.insert("id".to_string(), "1".to_string());
    /// params.insert("first_name".to_string(), "a name".to_string());
    /// params.insert("last_name".to_string(), "a first name".to_string());
    /// params.insert("email".to_string(), "tests@gg.com".to_string());
    /// params.insert("address".to_string(), "test street".to_string());
    /// params.insert("age".to_string(), "18".to_string());
    ///
    /// match check::validate_members_params(&params) {
    ///     Ok(..) => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    /// }
    /// ```
    ///
    /// ```
    /// use lib_api::utils::check;
    /// use std::collections::HashMap;
    ///
    /// let mut params = HashMap::new();
    /// params.insert("id".to_string(), "1".to_string());
    /// params.insert("ids".to_string(), "1,2".to_string());
    /// params.insert("first_name".to_string(), "a name".to_string());
    /// params.insert("last_name".to_string(), "a first name".to_string());
    /// params.insert("email".to_string(), "tests@gg.com".to_string());
    /// params.insert("address".to_string(), "test street".to_string());
    /// params.insert("age".to_string(), "18".to_string());
    ///
    /// match check::validate_members_params(&params) {
    ///     Err(e) if e.to_string() == "select only one of them, id xor ids" => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    ///     Ok(_) => panic!("Returned an Ok variant!"),
    /// }
    /// ```
    ///
    /// ```
    /// use lib_api::utils::check;
    /// use std::collections::HashMap;
    ///
    /// let mut params = HashMap::new();
    /// params.insert("ids".to_string(), "1,2".to_string());
    /// params.insert("first_name".to_string(), "a name".to_string());
    /// params.insert("last_name".to_string(), "a first name".to_string());
    /// params.insert("email".to_string(), "tests@gg.com".to_string());
    /// params.insert("address".to_string(), "test street".to_string());
    /// params.insert("age".to_string(), "a".to_string());
    ///
    /// match check::validate_members_params(&params) {
    ///     Err(e) if e.to_string() == "Error parsing string: 'a', not a valid integer" => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    ///     Ok(_) => panic!("Returned an Ok variant!"),
    /// }
    ///```
    pub fn validate_members_params(params: &HashMap<String, String>) -> Result<bool, CustomError> {
        let keys = vec![
            "id".to_string(),
            "ids".to_string(),
            "first_name".to_string(),
            "last_name".to_string(),
            "email".to_string(),
            "address".to_string(),
            "age".to_string(),
        ];

        for key in params.keys() {
            if !keys.contains(key) {
                return Err(CustomError::new(
                    400,
                    format!("the parameter '{key}' is incorrect"),
                ));
            }
        }

        if params.get("id").is_some() && params.get("ids").is_some() {
            return Err(CustomError::new(
                400,
                "select only one of them, id xor ids".to_string(),
            ));
        }

        if let Some(id) = params.get("id") {
            match validate_int(id) {
                Ok(..) => (),
                Err(err) => return Err(err),
            }
        }

        if let Some(ids) = params.get("ids") {
            match parse_ids(ids) {
                Ok(..) => (),
                Err(err) => return Err(err),
            }
        }

        if let Some(age) = params.get("age") {
            match validate_int(age) {
                Ok(..) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(true)
    }

    /// Check if a params for book are correct.
    ///
    /// pub struct Member {
    ///     pub id: i32,
    ///     pub title: String,
    ///     pub isbn: String,
    ///     pub copies_available: i32,
    ///     pub copies: i32,
    /// }
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_api::utils::check;
    /// use std::collections::HashMap;
    ///
    /// let mut params = HashMap::new();
    /// params.insert("id".to_string(), "1".to_string());
    /// params.insert("title".to_string(), "a title".to_string());
    /// params.insert("isbn".to_string(), "1234".to_string());
    /// params.insert("copies_available".to_string(), "3".to_string());
    /// params.insert("copies".to_string(), "3".to_string());
    ///
    /// match check::validate_book_params(&params) {
    ///     Ok(..) => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    /// }
    /// ```
    ///
    /// ```
    /// use lib_api::utils::check;
    /// use std::collections::HashMap;
    ///
    /// let mut params = HashMap::new();
    /// params.insert("ids".to_string(), "1,2".to_string());
    /// params.insert("id".to_string(), "1".to_string());
    /// params.insert("title".to_string(), "a title".to_string());
    /// params.insert("isbn".to_string(), "1234".to_string());
    /// params.insert("copies_available".to_string(), "3".to_string());
    /// params.insert("copies".to_string(), "3".to_string());
    ///
    /// match check::validate_book_params(&params) {
    ///     Err(e) if e.to_string() == "select only one of them, id xor ids" => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    ///     Ok(_) => panic!("Returned an Ok variant!"),
    /// }
    /// ```
    pub fn validate_book_params(params: &HashMap<String, String>) -> Result<bool, CustomError> {
        let keys = vec![
            "id".to_string(),
            "ids".to_string(),
            "title".to_string(),
            "isbn".to_string(),
            "copies_available".to_string(),
            "copies".to_string(),
        ];

        for key in params.keys() {
            if !keys.contains(key) {
                return Err(CustomError::new(
                    400,
                    format!("the parameter '{key}' is incorrect"),
                ));
            }
        }

        if params.get("id").is_some() && params.get("ids").is_some() {
            return Err(CustomError::new(
                400,
                "select only one of them, id xor ids".to_string(),
            ));
        }

        if let Some(id) = params.get("id") {
            match validate_int(id) {
                Ok(..) => (),
                Err(err) => return Err(err),
            }
        }

        if let Some(ids) = params.get("ids") {
            match parse_ids(ids) {
                Ok(..) => (),
                Err(err) => return Err(err),
            }
        }

        if let Some(copies_available) = params.get("copies_available") {
            match validate_int(copies_available) {
                Ok(..) => (),
                Err(err) => return Err(err),
            }
        }

        if let Some(copies) = params.get("copies") {
            match validate_int(copies) {
                Ok(..) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(true)
    }
}

// // Alternative tests
// #[cfg(test)]
// mod test {
//     use std::collections::HashMap;
//
//     use crate::utils::check;
//
//     #[test]
//     fn validate_int_test_ok() {
//         match check::validate_int("2") {
//             Ok(n) => assert_eq!(2, n),
//             Err(e) => panic!("Returned Err! => {e}"),
//         }
//     }
//
//     #[test]
//     fn validate_int_test_ko() {
//         match check::validate_int("a") {
//             Err(e) if e.to_string() == "Error parsing string: 'a', not a valid integer" => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//             Ok(_) => panic!("Returned an Ok variant!"),
//         }
//     }
//
//     #[test]
//     fn validate_float_test_ok() {
//         match check::validate_float("1.1") {
//             Ok(n) => assert_eq!(1.1, n),
//             Err(e) => panic!("Returned Err! => {e}"),
//         }
//     }
//
//     #[test]
//     fn validate_float_test_ko() {
//         match check::validate_float("a") {
//             Err(e) if e.to_string() == "Error parsing string: 'a', not a valid float" => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//             Ok(_) => panic!("Returned an Ok variant!"),
//         }
//     }
//
//     #[test]
//     fn validate_ids_test_ok() {
//         match check::parse_ids("1,2") {
//             Ok(vec_n) => assert_eq!(vec![1, 2], vec_n),
//             Err(e) => panic!("Returned Err! => {e}"),
//         }
//     }
//
//     #[test]
//     fn validate_ids_test_ko() {
//         match check::parse_ids("a,1") {
//             Err(e) if e.to_string() == "Error parsing string: 'a', not a valid integer" => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//             Ok(_) => panic!("Returned an Ok variant!"),
//         }
//     }
//
//     #[test]
//     fn validate_members_params_test_ok() {
//         let mut params = HashMap::new();
//         params.insert("id".to_string(), "1".to_string());
//         params.insert("first_name".to_string(), "a name".to_string());
//         params.insert("last_name".to_string(), "a first name".to_string());
//         params.insert("email".to_string(), "tests@gg.com".to_string());
//         params.insert("address".to_string(), "test street".to_string());
//         params.insert("age".to_string(), "18".to_string());
//
//         match check::validate_members_params(&params) {
//             Ok(..) => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//         }
//     }
//
//     #[test]
//     fn validate_members_params_test_ko_ids() {
//         let mut params = HashMap::new();
//         params.insert("id".to_string(), "1".to_string());
//         params.insert("ids".to_string(), "1,2".to_string());
//         params.insert("first_name".to_string(), "a name".to_string());
//         params.insert("last_name".to_string(), "a first name".to_string());
//         params.insert("email".to_string(), "tests@gg.com".to_string());
//         params.insert("address".to_string(), "test street".to_string());
//         params.insert("age".to_string(), "18".to_string());
//
//         match check::validate_members_params(&params) {
//             Err(e) if e.to_string() == "select only one of them, id xor ids" => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//             Ok(_) => panic!("Returned an Ok variant!"),
//         }
//     }
//
//     #[test]
//     fn validate_members_params_test_ko_age() {
//         let mut params = HashMap::new();
//         params.insert("ids".to_string(), "1,2".to_string());
//         params.insert("first_name".to_string(), "a name".to_string());
//         params.insert("last_name".to_string(), "a first name".to_string());
//         params.insert("email".to_string(), "tests@gg.com".to_string());
//         params.insert("address".to_string(), "test street".to_string());
//         params.insert("age".to_string(), "a".to_string());
//
//         match check::validate_members_params(&params) {
//             Err(e) if e.to_string() == "Error parsing string: 'a', not a valid integer" => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//             Ok(_) => panic!("Returned an Ok variant!"),
//         }
//     }
//
//     #[test]
//     fn validate_book_params_test_ok() {
//         let mut params = HashMap::new();
//         params.insert("id".to_string(), "1".to_string());
//         params.insert("title".to_string(), "a title".to_string());
//         params.insert("isbn".to_string(), "1234".to_string());
//         params.insert("copies_available".to_string(), "3".to_string());
//         params.insert("copies".to_string(), "3".to_string());
//
//         match check::validate_book_params(&params) {
//             Ok(..) => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//         }
//     }
//
//     #[test]
//     fn validate_book_params_test_ko_ids() {
//         let mut params = HashMap::new();
//         params.insert("id".to_string(), "1".to_string());
//         params.insert("ids".to_string(), "1,2".to_string());
//         params.insert("title".to_string(), "a title".to_string());
//         params.insert("isbn".to_string(), "1234".to_string());
//         params.insert("copies_available".to_string(), "3".to_string());
//         params.insert("copies".to_string(), "3".to_string());
//
//         match check::validate_book_params(&params) {
//             Err(e) if e.to_string() == "select only one of them, id xor ids" => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//             Ok(_) => panic!("Returned an Ok variant!"),
//         }
//     }
// }
