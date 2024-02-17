use serde::Serialize;

pub mod query {
    use bson;
    use serde::Serialize;

    pub trait Query /* : /*Serialize +*/ Into<bson::Bson> */ {
        fn into_q(self) -> bson::Document;
    }

    #[derive(Debug, Serialize)]
    pub enum Comparison<T> {
        Eq(T),
        Ne(T),
        Gt(T),
        Gte(T),
        Lt(T),
        Lte(T),
        In(Vec<T>),
        Nin(Vec<T>),
    }

    impl<T: Into<bson::Bson>> Comparison<T> {
        pub fn into_query(self) -> bson::Document {
            let x = match self {
                Comparison::Eq(value) => bson::doc! { "$eq": value },
                Comparison::Ne(value) => bson::doc! { "$ne": value },
                Comparison::Gt(value) => bson::doc! { "$gt": value },
                Comparison::Lt(value) => bson::doc! { "$lt": value },
                Comparison::Gte(value) => bson::doc! {"$gte": value},
                Comparison::Lte(value) => bson::doc! {"$lte": value},
                Comparison::In(value) => bson::doc! { "$in": value },
                Comparison::Nin(value) => bson::doc! { "$nin": value },
            };
            x
        }
    }

    impl<T: Into<bson::Bson>> Query for Comparison<T> {
        fn into_q(self) -> bson::Document {
            let x = match self {
                Comparison::Eq(value) => bson::doc! { "$eq": value },
                Comparison::Ne(value) => bson::doc! { "$ne": value },
                Comparison::Gt(value) => bson::doc! { "$gt": value },
                Comparison::Lt(value) => bson::doc! { "$lt": value },
                Comparison::Gte(value) => bson::doc! {"$gte": value},
                Comparison::Lte(value) => bson::doc! {"$lte": value},
                Comparison::In(value) => bson::doc! { "$in": value },
                Comparison::Nin(value) => bson::doc! { "$nin": value },
            };
            x
        }
    }

    #[derive(Debug)]
    pub enum Logical<T> {
        And(Vec<T>),
        Not(T),
        Or(Vec<T>),
        Nor(Vec<T>),
    }

    impl<T: Serialize + Into<bson::Bson>> Logical<T> {
        pub fn into_query(self) -> bson::Document {
            let x = match self {
                Logical::And(value) => bson::doc! { "$and": value },
                Logical::Not(value) => bson::doc! { "$not": value },
                Logical::Or(value) => bson::doc! { "$or": value },
                Logical::Nor(value) => bson::doc! { "$nor": value },
            };
            x
        }
    }

    impl<T: Into<bson::Bson>> Query for Logical<T> {
        fn into_q(self) -> bson::Document {
            let x = match self {
                Logical::And(value) => bson::doc! { "$and": value },
                Logical::Not(value) => bson::doc! { "$not": value },
                Logical::Or(value) => bson::doc! { "$or": value },
                Logical::Nor(value) => bson::doc! { "$nor": value },
            };
            x
        }
    }
}

#[cfg(test)]
mod tests {
    use bson;
    use query::Comparison;
    use query::Logical;
    use serde::{Deserialize, Serialize};

    use self::query::Query;

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct Student {
        id: u32,
        name: String,
        surname: String,
    }

    #[derive(Debug, Default)]
    struct StudentQuery {
        id: Option<Comparison<u32>>,
        name: Option<Comparison<String>>,
        surname: Option<Comparison<String>>,
    }

    impl Query for StudentQuery {
        fn into_q(self) -> bson::Document {
            let mut query = bson::doc! {};

            match self.id {
                Some(id) => {
                    query.insert("id", id.into_query());
                }
                None => {}
            }

            match self.name {
                Some(name) => {
                    query.insert("name", name.into_query());
                }
                None => {}
            }

            match self.surname {
                Some(surname) => {
                    query.insert("surname", surname.into_query());
                }
                None => {}
            }

            query
        }
    }

    impl StudentQuery {
        pub fn with_id(self, id: Comparison<u32>) -> Self {
            Self {
                id: Some(id),
                ..self
            }
        }
        pub fn with_name(self, name: Comparison<String>) -> Self {
            Self {
                name: Some(name),
                ..self
            }
        }

        pub fn with_surname(self, surname: Comparison<String>) -> Self {
            Self {
                surname: Some(surname),
                ..self
            }
        }
    }

    #[test]
    fn it_works() {
        let student_q = StudentQuery::default()
            .with_id(Comparison::Gt(1))
            .with_name(Comparison::Eq("John".to_string()))
            .with_surname(Comparison::Eq("Doe".to_string()));

        let query = student_q.into_q();
        println!("{:?}", query);

        let student_q = StudentQuery::default().with_id(Comparison::Gt(1));

        let query = student_q.into_q();
        println!("{:?}", query);

        let student_id = StudentQuery::default().with_id(Comparison::Gt(1));
        let student_name = StudentQuery::default().with_name(Comparison::Eq("John".to_string()));
        let or = Logical::Or(vec![student_id.into_q(), student_name.into_q()]);
        let query = or.into_q();
        println!("{:?}", query);
    }
}
