use serde::Serialize;

pub use bson;
pub use serde;

pub mod query {
    use bson;
    use serde::Serialize;

    pub trait Parameter {
        fn to_bson(self) -> bson::Document;
    }

    pub trait Document: Parameter {
        type ResponseDocument; /* : DeserializeOwned*/
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

    impl<T: Serialize> Parameter for Comparison<T> {
        fn to_bson(self) -> bson::Document {
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

    impl<T: Document + crate::Serialize> Document for Comparison<T> {
        type ResponseDocument = T::ResponseDocument;
    }

    #[derive(Debug, Serialize)]
    pub enum Logical<T> {
        Not(T),
        And(Vec<LogicalParameter<T>>),
        Or(Vec<LogicalParameter<T>>),
        Nor(Vec<LogicalParameter<T>>),
    }

    impl<T: Serialize> Parameter for Logical<T> {
        fn to_bson(self) -> bson::Document {
            let x = match self {
                Logical::And(value) => bson::doc! { "$and": bson::ser::to_bson(&value).unwrap() },
                Logical::Not(value) => bson::doc! { "$not": bson::ser::to_bson(&value).unwrap() },
                Logical::Or(value) => bson::doc! { "$or": bson::ser::to_bson(&value).unwrap() },
                Logical::Nor(value) => bson::doc! { "$nor": bson::ser::to_bson(&value).unwrap() },
            };
            x
        }
    }

    impl<T: Document + crate::Serialize> Document for Logical<T> {
        type ResponseDocument = T::ResponseDocument;
    }

    #[derive(Debug, Serialize)]
    #[serde(untagged)] // we do not want them in the final query
    pub enum LogicalParameter<T> {
        Value(T),
        Logical(Logical<T>),
    }

    impl<T: Serialize> Parameter for LogicalParameter<T> {
        fn to_bson(self) -> bson::Document {
            let x = match self {
                LogicalParameter::Logical(value) => value.to_bson(),
                LogicalParameter::Value(value) => bson::ser::to_bson(&value)
                    .unwrap()
                    .as_document()
                    .unwrap()
                    .clone(),
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
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};

    use crate::query::Document;
    use crate::query::LogicalParameter;

    use self::query::Parameter;

    use super::*;

    #[derive(Debug, Default, Serialize, Deserialize)]
    struct Student {
        // #[mongo_q(query, index)]
        id: u32,
        // #[mongo_q(query)]
        name: String,
        surname: String,
    }

    #[derive(Debug, Default, Serialize)]
    struct StudentQuery {
        id: Option<Comparison<u32>>,
        name: Option<Comparison<String>>,
        surname: Option<Comparison<String>>,
    }

    impl Parameter for StudentQuery {
        fn to_bson(self) -> bson::Document {
            let mut query = bson::doc! {};

            match self.id {
                Some(id) => {
                    query.insert("id", id.to_bson());
                }
                None => {}
            }

            match self.name {
                Some(name) => {
                    query.insert("name", name.to_bson());
                }
                None => {}
            }

            match self.surname {
                Some(surname) => {
                    query.insert("surname", surname.to_bson());
                }
                None => {}
            }

            query
        }
    }

    impl Document for StudentQuery {
        type ResponseDocument = Student;
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

    #[derive(Debug, Serialize, Deserialize, Default)]
    struct Class {
        students: Vec<Student>,
    }

    #[derive(Debug, Serialize, Default)]
    struct ClassQuery {
        students: Option<Logical<StudentQuery>>,
    }

    impl Parameter for ClassQuery {
        fn to_bson(self) -> bson::Document {
            let mut query = bson::doc! {};

            match self.students {
                Some(students) => {
                    query.insert("students", students.to_bson());
                }
                None => {}
            }

            query
        }
    }

    impl Document for ClassQuery {
        type ResponseDocument = Class;
    }

    #[test]
    fn it_works() {
        let student_q = StudentQuery::default()
            .with_id(Comparison::Gt(1))
            .with_name(Comparison::Eq("John".to_string()))
            .with_surname(Comparison::Eq("Doe".to_string()));

        let query = student_q.to_bson();
        println!("{:?}", query);

        let student_q = StudentQuery::default().with_id(Comparison::Gt(1));

        let query = student_q.to_bson();
        println!("{:?}", query);

        let student_id = StudentQuery::default().with_id(Comparison::Gt(1));
        let student_name = StudentQuery::default().with_name(Comparison::Eq("John".to_string()));
        let or = Logical::Or(vec![
            LogicalParameter::Value(student_id),
            LogicalParameter::Value(student_name),
        ]);
        let query = or.to_bson();
        println!("{:?}", query);
    }

    #[test]
    fn test_vec_of_students() {
        let class_q = ClassQuery {
            students: Some(Logical::Or(vec![
                LogicalParameter::Value(StudentQuery::default().with_id(Comparison::Gt(1))),
                LogicalParameter::Value(
                    StudentQuery::default().with_name(Comparison::Eq("John".to_string())),
                ),
            ])),
        };
        println!("{:?}", class_q.to_bson());
    }

    #[test]
    fn test_fn_receive_q() {
        // how to add result?
        fn foo<T: Parameter>(q: T) {
            println!("{:?}", q.to_bson());
        }
        let student_q = StudentQuery::default().with_id(Comparison::Gt(1));
        foo(student_q);
    }

    #[test]
    fn test_fn_receive_q_return_doc() {
        // how to add result?
        fn query_document<T>(q: T) -> T::ResponseDocument
        where
            T: Document,
            // Only for tests
            T::ResponseDocument: Default + Serialize + DeserializeOwned,
        {
            let _query = q.to_bson();
            println!("{:?}", _query);
            let response = T::ResponseDocument::default();
            let d = bson::ser::to_document(&response).unwrap();

            bson::from_document(d).unwrap()
        }

        let student_q = StudentQuery::default().with_id(Comparison::Gt(1));
        query_document(student_q);

        let class_q = ClassQuery {
            students: Some(Logical::Or(vec![
                LogicalParameter::Value(StudentQuery::default().with_id(Comparison::Gt(1))),
                LogicalParameter::Value(
                    StudentQuery::default().with_name(Comparison::Eq("John".to_string())),
                ),
            ])),
        };

        query_document(class_q);

        let student_id = StudentQuery::default().with_id(Comparison::Gt(1));
        let student_name = StudentQuery::default().with_name(Comparison::Eq("John".to_string()));
        let or = Logical::Or(vec![
            LogicalParameter::Value(student_id),
            LogicalParameter::Value(student_name),
        ]);
        let _x = query_document(or);

        let student_id = StudentQuery::default().with_id(Comparison::Gt(1));
        let student_name = StudentQuery::default().with_name(Comparison::Eq("John".to_string()));
        let and = Logical::And(vec![
            LogicalParameter::Value(student_id),
            LogicalParameter::Value(student_name),
        ]);
        let _x = query_document(and);

        let class_q = ClassQuery {
            students: Some(Logical::Or(vec![
                LogicalParameter::Value(StudentQuery::default().with_id(Comparison::Gt(5))),
                LogicalParameter::Logical(Logical::And(vec![
                    LogicalParameter::Value(StudentQuery::default().with_id(Comparison::Eq(1))),
                    LogicalParameter::Value(
                        StudentQuery::default().with_name(Comparison::Eq("John".to_string())),
                    ),
                ])),
            ])),
        };

        query_document(class_q);
    }

    #[test]
    fn test_fn_delete_doc() {
        // how to add result?
        fn delete_document<T>(q: T)
        where
            T: Document,
        {
            let _query = q.to_bson();
            println!("{:?}", _query);
        }

        let student_q = StudentQuery::default().with_id(Comparison::Gt(1));
        delete_document(student_q);
    }

    #[test]
    fn test_determenistic_order() {
        #[derive(Debug, Serialize, Deserialize, Default)]
        struct BlockNumber(i64);

        #[derive(Debug, Serialize, Deserialize, Default)]
        struct Block {
            number: BlockNumber,
            hash: Option<String>,
        }

        #[derive(Debug, Serialize, Default)]
        struct BlockQuery {
            number: Option<Comparison<BlockNumber>>,
            hash: Option<Comparison<Option<String>>>,
        }

        impl Parameter for BlockQuery {
            fn to_bson(self) -> bson::Document {
                let mut query = bson::doc! {};

                match self.number {
                    Some(block_number) => {
                        query.insert("number", block_number.to_bson());
                    }
                    None => {}
                }

                match self.hash {
                    Some(hash) => {
                        query.insert("hash", hash.to_bson());
                    }
                    None => {}
                }

                query
            }
        }

        let q1 = BlockQuery {
            number: Some(Comparison::Gt(BlockNumber(1))),
            hash: Some(Comparison::Eq(Some("0x123".to_string()))),
        };

        let q2 = BlockQuery {
            hash: Some(Comparison::Eq(Some("0x123".to_string()))),
            number: Some(Comparison::Gt(BlockNumber(1))),
        };

        assert_eq!(q1.to_bson().to_string(), q2.to_bson().to_string());
    }
}
