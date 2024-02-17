pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait Query {}

#[cfg(test)]
mod tests {
    use bson;
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct Student {
        id: u32,
        name: String,
        surname: String,
    }

    #[derive(Debug)]
    enum Comparison<T> {
        Eq(T),
        Ne(T),
        Gt(T),
        Gte(T),
        Lt(T),
        Lte(T),
        In(Vec<T>),
        Nin(Vec<T>),
    }

    impl<T: Serialize + Into<bson::Bson>> Comparison<T> {
        fn into_query(self) -> bson::Document {
            let (comparison, value) = match self {
                Comparison::Eq(value) => ("$eq", value),
                Comparison::Ne(value) => ("$ne", value),
                Comparison::Gt(value) => ("$gt", value),
                Comparison::Lt(value) => ("$lt", value),
                Comparison::Gte(value) => ("$gte", value),
                Comparison::Lte(value) => ("$lte", value),
                // Comparison::In(value) => ("$in", value),
                _ => panic!("Not implemented"),
            };
            let x = bson::doc! { comparison: value };
            x
        }
    }

    #[derive(Debug, Default)]
    struct StudentQuery {
        id: Option<Comparison<u32>>,
        name: Option<Comparison<String>>,
        surname: Option<Comparison<String>>,
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

        fn into_query(self) -> bson::Document {
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

    #[test]
    fn it_works() {
        let student_q = StudentQuery::default()
            .with_id(Comparison::Gt(1))
            .with_name(Comparison::Eq("John".to_string()))
            .with_surname(Comparison::Eq("Doe".to_string()));

        let query = student_q.into_query();

        let student_q = StudentQuery::default().with_id(Comparison::Gt(1));

        let query = student_q.into_query();
        println!("{:?}", query);
    }
}
