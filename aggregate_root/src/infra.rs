use sqlx::SqliteConnection;

use crate::domain::{self, entities::UserAggregateRoot, infra_ifs};

#[derive(Debug, Clone, PartialEq)]
struct FetchUserByIdRow {
    pub user_id: i64,
    pub user_name: String,
    pub user_updated_datetime: chrono::NaiveDateTime,
    pub user_created_datetime: chrono::NaiveDateTime,
    pub department_id: Option<i64>,
    pub department_name: Option<String>,
    pub department_updated_datetime: Option<chrono::NaiveDateTime>,
    pub department_created_datetime: Option<chrono::NaiveDateTime>,
}

pub struct UserAggregateRepository(SqliteConnection);
impl infra_ifs::UserAggregateRepository for UserAggregateRepository {
    async fn find_user_by_id(&mut self, id: i32) -> anyhow::Result<Vec<UserAggregateRoot>> {
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
        .fetch_all(&mut self.0)
        .await?;

        let rows = rows
            .into_iter()
            .fold(vec![], |mut acc: Vec<UserAggregateRoot>, row| {
                if let Some(user) = acc.iter_mut().find(|user| user.id == row.user_id) {
                    if let Some(department_id) = row.department_id {
                        user.departments.push(domain::entities::Department {
                            id: department_id,
                            user_id: row.user_id,
                            name: row.department_name.unwrap_or_default(),
                            updated_datetime: row.department_updated_datetime.unwrap_or_default(),
                            created_datetime: row.department_created_datetime.unwrap_or_default(),
                        });
                    }
                } else {
                    let new_user = UserAggregateRoot {
                        id: row.user_id,
                        name: row.user_name,
                        updated_datetime: row.user_updated_datetime,
                        created_datetime: row.user_created_datetime,
                        departments: if let Some(department_id) = row.department_id {
                            vec![domain::entities::Department {
                                id: department_id,
                                user_id: row.user_id,
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

    async fn create_user(&mut self, name: String) -> anyhow::Result<i64> {
        let created_datetime = chrono::Utc::now().naive_utc();
        let updated_datetime = created_datetime;

        let id = sqlx::query!(
            r#"
                    INSERT INTO users (name, updated_datetime, created_datetime)
                    VALUES ($1, $2, $3)
                "#,
            name,
            created_datetime,
            updated_datetime
        )
        .execute(&mut self.0)
        .await?
        .last_insert_rowid();

        Ok(id)
    }
}
