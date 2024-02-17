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

    impl<T: Serialize> Query for Comparison<T> {
        // impl<T: Into<bson::Bson>> Query for Comparison<T> {
        fn into_q(self) -> bson::Document {
            let x = match self {
                Comparison::Eq(value) => bson::doc! { "$eq": bson::ser::to_bson(&value).unwrap() },
                Comparison::Ne(value) => bson::doc! { "$ne": bson::ser::to_bson(&value).unwrap() },
                Comparison::Gt(value) => bson::doc! { "$gt": bson::ser::to_bson(&value).unwrap() },
                Comparison::Lt(value) => bson::doc! { "$lt": bson::ser::to_bson(&value).unwrap() },
                Comparison::Gte(value) => {
                    bson::doc! { "$gte": bson::ser::to_bson(&value).unwrap() }
                }
                Comparison::Lte(value) => {
                    bson::doc! { "$lte": bson::ser::to_bson(&value).unwrap() }
                }
                Comparison::In(value) => bson::doc! { "$in": bson::ser::to_bson(&value).unwrap() },
                Comparison::Nin(value) => {
                    bson::doc! { "$nin": bson::ser::to_bson(&value).unwrap() }
                }
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

    impl<T: Serialize> Query for Logical<T> {
        // impl<T: Into<bson::Bson>> Query for Logical<T> {
        fn into_q(self) -> bson::Document {
            let x = match self {
                Logical::And(value) => bson::doc! { "$and": bson::ser::to_bson(&value).unwrap() },
                Logical::Not(value) => bson::doc! { "$not": bson::ser::to_bson(&value).unwrap() },
                Logical::Or(value) => bson::doc! { "$or": bson::ser::to_bson(&value).unwrap() },
                Logical::Nor(value) => bson::doc! { "$nor": bson::ser::to_bson(&value).unwrap() },
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

    #[derive(Debug, Default, Serialize)]
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
                    query.insert("id", id.into_q());
                }
                None => {}
            }

            match self.name {
                Some(name) => {
                    query.insert("name", name.into_q());
                }
                None => {}
            }

            match self.surname {
                Some(surname) => {
                    query.insert("surname", surname.into_q());
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

    #[test]
    fn test_vec_of_students() {
        #[derive(Debug, Serialize)]
        struct Class {
            students: Vec<Student>,
        }

        #[derive(Debug, Serialize)]
        struct ClassQuery {
            // students: Option<Logical<StudentQuery>>,
            students: Option<Comparison<StudentQuery>>,
        }

        impl Query for ClassQuery {
            fn into_q(self) -> bson::Document {
                let mut query = bson::doc! {};

                match self.students {
                    Some(students) => {
                        query.insert("students", students.into_q());
                    }
                    None => {}
                }

                query
            }
        }

        // impl Into<bson::Bson> for Comparison<StudentQuery> {
        //     fn into(self) -> bson::Bson {
        //         match self {
        //             Comparison::Eq(value) => bson::Bson::from(value),
        //             Comparison::Gt(value) => bson::Bson::from(value),
        //             Comparison::Lt(value) => bson::Bson::from(value),
        //             _ => panic!("saaa");
        //             // Add more variants if necessary
        //         }
        //     }
        // }
    }
}
