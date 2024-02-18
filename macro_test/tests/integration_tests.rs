#[cfg(test)]
mod tests {

    use typed_mongo_query_macro::Queriable;
    fn test_basic_struct() {
        let input = r#"
            #[derive(Queriable)]
            struct Test {
                id: i32,
                name: String,
                age: i32,
            }
        "#;

        let expected = r#"
            struct TestQuery {
                id: Option<Comparison<i32>>,
                name: Option<Comparison<String>>,
                age: Option<Comparison<i32>>,
            }

            impl TestQuery {
                pub fn with_id(mut self, value: Comparison<i32>) -> Self {
                    self.id = Some(value);
                    self
                }

                pub fn with_name(mut self, value: Comparison<String>) -> Self {
                    self.name = Some(value);
                    self
                }

                pub fn with_age(mut self, value: Comparison<i32>) -> Self {
                    self.age = Some(value);
                    self
                }
            }
        "#;
    }
}
