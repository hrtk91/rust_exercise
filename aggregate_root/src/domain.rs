pub mod entities {
    #[derive(Debug, Clone, PartialEq)]
    pub struct UserAggregateRoot {
        pub id: i64,
        pub name: String,
        pub updated_datetime: chrono::NaiveDateTime,
        pub created_datetime: chrono::NaiveDateTime,
        pub departments: Vec<Department>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Department {
        pub id: i64,
        pub user_id: i64,
        pub name: String,
        pub updated_datetime: chrono::NaiveDateTime,
        pub created_datetime: chrono::NaiveDateTime,
    }
}

pub mod infra_ifs {
    use std::future::Future;

    pub trait UserAggregateRepository {
        fn find_user_by_id(
            &mut self,
            id: i64,
        ) -> impl Future<Output = anyhow::Result<Vec<super::entities::UserAggregateRoot>>>;
        fn create_user(
            &mut self,
            user_name: String,
            department_name: Option<String>,
        ) -> impl Future<Output = anyhow::Result<i64>>;
    }
}
