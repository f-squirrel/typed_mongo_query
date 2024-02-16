pub fn add(left: usize, right: usize) -> usize {
    left + right
}

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
        Equal(T),
        NotEqual(T),
        GreaterThan(T),
        LessThan(T),
        GreaterThanOrEqual(T),
        LessThanOrEqual(T),
    }

    impl<T: Serialize + Into<bson::Bson>> Comparison<T> {
        fn into_query(self) -> bson::Document {
            let (comparison, value) = match self {
                Comparison::Equal(value) => ("$eq", value),
                Comparison::NotEqual(value) => ("$ne", value),
                Comparison::GreaterThan(value) => ("$gt", value),
                Comparison::LessThan(value) => ("$lt", value),
                Comparison::GreaterThanOrEqual(value) => ("$gte", value),
                Comparison::LessThanOrEqual(value) => ("$lte", value),
            };

            bson::doc! { comparison: value }
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
            .with_id(Comparison::GreaterThan(1))
            .with_name(Comparison::Equal("John".to_string()))
            .with_surname(Comparison::Equal("Doe".to_string()));

        let query = student_q.into_query();
        println!("{:?}", query);
    }
}
