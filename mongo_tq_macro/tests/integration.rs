#[cfg(test)]
mod tests {

    use mongo_tq::query::Comparison;
    use mongo_tq::query::Document;
    use mongo_tq::query::Logical;
    use mongo_tq::query::LogicalParameter;
    use mongo_tq::query::Parameter;
    use mongo_tq_macro::Queryable;

    #[test]
    fn test_basic_struct() {
        #[derive(/* Serialize,  */ Queryable)]
        struct Student {
            #[mongo_tq(cmp)]
            id: i32,
            #[mongo_tq(cmp)]
            name: String,
            age: i32,
            #[mongo_tq(cmp)]
            info: Info,
        }

        #[derive(Queryable, mongo_tq::serde::Serialize)]
        struct Info {
            #[mongo_tq(cmp)]
            field1: i32,
            field2: String,
        }

        let q = StudentQuery::all()
            .with_name(Comparison::Eq("little zayec".to_string()))
            .with_id(Comparison::Gt(12));

        let q2 = StudentQuery::all()
            .with_name(Comparison::Eq("bla".to_string()))
            .with_id(Comparison::Lt(12));

        let or_q = Logical::Or(vec![
            LogicalParameter::Value(q),
            LogicalParameter::Value(q2),
        ]);
        // or_q.to_bson();

        let _query = StudentQuery::all()
            .with_id(Comparison::Eq(1))
            .with_name(Comparison::Eq("John".to_string()));

        let raw_q = _query.to_bson();
        println!("RW: {:?}", raw_q);

        // let student_query = StudentQuery::all().with_info(Comparison::Eq(
        //     InfoQuery::all().with_field1(Comparison::Eq(1)),
        // ));
    }
}
