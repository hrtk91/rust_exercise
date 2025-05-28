pub mod queries {
    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct FetchUserByIdQuery {
        pub user_id: i64,
    }
}

pub mod commands {
    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct CreateUser {
        pub user_name: String,
        pub department_name: Option<String>,
    }
}

pub mod dtos {
    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct User {
        pub id: i64,
        pub name: String,
        pub updated_datetime: chrono::NaiveDateTime,
        pub created_datetime: chrono::NaiveDateTime,
        pub departments: Vec<Department>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct Department {
        pub id: i64,
        pub user_id: i64,
        pub name: String,
        pub updated_datetime: chrono::NaiveDateTime,
        pub created_datetime: chrono::NaiveDateTime,
    }
}

use crate::kernel::interfaces::{self};

pub async fn fetch_user_by_id(
    mut repo: impl interfaces::UserAggregateRepository,
    payload: queries::FetchUserByIdQuery,
) -> anyhow::Result<Vec<dtos::User>> {
    let result = repo.find_user_by_id(payload.user_id).await?;

    Ok(result
        .iter()
        .map(|x| dtos::User {
            id: x.id,
            name: x.name.clone(),
            departments: x
                .departments
                .iter()
                .map(|x| dtos::Department {
                    id: x.id,
                    name: x.name.clone(),
                    user_id: x.user_id,
                    updated_datetime: x.updated_datetime,
                    created_datetime: x.created_datetime,
                })
                .collect(),
            updated_datetime: x.updated_datetime,
            created_datetime: x.created_datetime,
        })
        .collect())
}

pub async fn create_user(
    mut repo: impl interfaces::UserAggregateRepository,
    payload: commands::CreateUser,
) -> anyhow::Result<i64> {
    let id = repo
        .create_user(payload.user_name, payload.department_name)
        .await?;
    Ok(id)
}

#[cfg(test)]
#[tokio::test]
async fn integration_test() {
    use std::str::FromStr;

    use crate::kernel::interfaces::UserAggregateRepository;
    use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

    // arrange
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::from_str("sqlite::memory:?cache=shared")
            .expect("DBオープンに失敗しました")
            .in_memory(true),
    )
    .await
    .expect("テストDB作成に失敗しました");
    {
        let mut conn = pool
            .acquire()
            .await
            .expect("arrange:コネクション取得に失敗");
        sqlx::migrate!()
            .run(&mut *conn)
            .await
            .expect("DB初期化に失敗しました");
    }
    let mut repo = crate::adapter::repository::UserAggregateRepository(pool);

    let id = repo
        .create_user("user1".into(), Some("dep1".into()))
        .await
        .expect("ユーザー作成失敗");

    match fetch_user_by_id(repo, queries::FetchUserByIdQuery { user_id: id }).await {
        Ok(values) => {
            assert_eq!(1, values.len());
            assert_eq!(1, values[0].id);
            assert_eq!("user1", values[0].name.as_str());
        }
        Err(_) => {
            assert!(false)
        }
    };
}