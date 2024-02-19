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
            #[mongo_tq(cmp)]
            id: i32,
            #[mongo_tq(cmp)]
            name: String,
            age: i32,
        }

        let _query = StudentQuery::all()
            .with_id(Comparison::Eq(1))
            .with_name(Comparison::Eq("John".to_string()));

        // let raw_q = _query.to_bson();
    }
}
