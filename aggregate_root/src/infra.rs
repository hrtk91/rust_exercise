use std::sync::LazyLock;

use sqlx::{Acquire, Pool, SqlitePool};

use crate::domain::{self, entities::UserAggregateRoot, infra_ifs};

pub static POOL: LazyLock<SqlitePool> = LazyLock::new(|| {
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_lazy(
            &std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string()),
        )
        .expect("Failed to create database connection pool")
});

#[derive(Debug, Clone, PartialEq)]
struct FetchUserByIdRow {
    pub user_id: Option<i64>,
    pub user_name: String,
    pub user_updated_datetime: chrono::NaiveDateTime,
    pub user_created_datetime: chrono::NaiveDateTime,
    pub department_id: Option<i64>,
    pub department_name: Option<String>,
    pub department_updated_datetime: Option<chrono::NaiveDateTime>,
    pub department_created_datetime: Option<chrono::NaiveDateTime>,
}

/// # sqlxメモ
/// ## Acquire
/// Executorを取得するためのacquire()メソッドのtrait  
/// Pool, PoolConnection等に実装されている
/// 所有権を取得するため、構造体に持ちまわす用途には使えない
/// そのため、UserAggregateRepositoryはPool<DB>のstruct自体を保持するようにした(DB可搬性を諦め)
/// ## Executor
/// クエリを実行するためのexecuteメソッドを備えるtrait
/// Transaction,Connection等に実装されている
pub struct UserAggregateRepository(pub Pool<sqlx::sqlite::Sqlite>);
impl UserAggregateRepository {
    pub fn new(pool: Pool<sqlx::sqlite::Sqlite>) -> Self {
        UserAggregateRepository(pool)
    }
}
impl infra_ifs::UserAggregateRepository for UserAggregateRepository {
    async fn find_user_by_id(&mut self, id: i64) -> anyhow::Result<Vec<UserAggregateRoot>> {
        let mut conn = self.0.acquire().await?;
        let rows = sqlx::query_as!(
            FetchUserByIdRow,
            r#"
                SELECT
                    users.id AS user_id,
                    users.name AS user_name,
                    users.updated_datetime AS user_updated_datetime,
                    users.created_datetime AS user_created_datetime,
                    departments.id AS department_id,
                    departments.name AS department_name,
                    departments.updated_datetime AS department_updated_datetime,
                    departments.created_datetime AS department_created_datetime
                FROM users
                LEFT JOIN departments ON users.id = departments.user_id
                WHERE user_id = $1
            "#,
            id
        )
        .fetch_all(&mut *conn)
        .await?;

        let rows = rows
            .into_iter()
            .fold(vec![], |mut acc: Vec<UserAggregateRoot>, row| {
                if let Some(user) = acc.iter_mut().find(|user| user.id == row.user_id.unwrap()) {
                    if let Some(department_id) = row.department_id {
                        user.departments.push(domain::entities::Department {
                            id: department_id,
                            user_id: row.user_id.unwrap(),
                            name: row.department_name.unwrap_or_default(),
                            updated_datetime: row.department_updated_datetime.unwrap_or_default(),
                            created_datetime: row.department_created_datetime.unwrap_or_default(),
                        });
                    }
                } else {
                    let new_user = UserAggregateRoot {
                        id: row.user_id.unwrap(),
                        name: row.user_name,
                        updated_datetime: row.user_updated_datetime,
                        created_datetime: row.user_created_datetime,
                        departments: if let Some(department_id) = row.department_id {
                            vec![domain::entities::Department {
                                id: department_id,
                                user_id: row.user_id.unwrap_or_default(),
                                name: row.department_name.unwrap_or_default(),
                                updated_datetime: row
                                    .department_updated_datetime
                                    .unwrap_or_default(),
                                created_datetime: row
                                    .department_created_datetime
                                    .unwrap_or_default(),
                            }]
                        } else {
                            vec![]
                        },
                    };
                    acc.push(new_user);
                }

                acc
            });

        Ok(rows)
    }

    async fn create_user(
        &mut self,
        user_name: String,
        department_name: Option<String>,
    ) -> anyhow::Result<i64> {
        let mut conn = self.0.acquire().await?;
        let created_datetime = chrono::Utc::now().naive_utc();
        let updated_datetime = created_datetime;

        sqlx::query!(r#"pragma foreign_keys = ON;"#)
            .execute(&mut *conn)
            .await?;

        if sqlx::query!(
            r#"
                select id from users where users.name == $1
            "#,
            user_name
        )
        .fetch_optional(&mut *conn)
        .await?
        .is_some()
        {
            // 重複エラー
            return Err(anyhow::anyhow!("400"));
        }

        if let Some(department_name) = department_name.clone() {
            if sqlx::query!(
                r#"
                    select id from departments where departments.name == $1
                "#,
                department_name
            )
            .fetch_optional(&mut *conn)
            .await?
            .is_some()
            {
                // 重複エラー
                return Err(anyhow::anyhow!("400"));
            }
        }

        let mut tx = conn.begin().await?;
        let user_id = sqlx::query!(
            r#"
                INSERT INTO users (name, updated_datetime, created_datetime)
                VALUES ($1, $2, $3)
            "#,
            user_name,
            updated_datetime,
            created_datetime,
        )
        .execute(&mut *tx)
        .await?
        .last_insert_rowid();

        sqlx::query!(
            r#"
                insert into departments (user_id, name, updated_datetime, created_datetime)
                values ($1, $2, $3, $4)
            "#,
            user_id,
            department_name,
            updated_datetime,
            created_datetime
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(user_id)
    }
}
