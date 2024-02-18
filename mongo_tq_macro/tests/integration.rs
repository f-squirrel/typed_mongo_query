#[cfg(test)]
mod tests {

    use mongo_tq::query::Comparison;
    use mongo_tq::query::Document;
    use mongo_tq::query::Parameter;
    use mongo_tq_macro::Queryable;

    #[test]
    fn test_basic_struct() {
        #[derive(Queryable)]
        struct Student {
            id: i32,
            name: String,
            age: i32,
        }

        let _query = StudentQuery {
            id: Some(Comparison::Eq(1)),
            name: None,
            age: None,
        };

        // let raw_q = _query.to_bson();
    }
}
