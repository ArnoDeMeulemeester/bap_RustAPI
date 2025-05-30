use actix_web::web;
use sqlx::mysql::MySqlRow;
use sqlx::MySqlPool;
use crate::models::user::User;

pub async fn find_all(pool: web::Data<MySqlPool>) -> Result<Vec<MySqlRow>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, first_name, last_name, password FROM user")
        .fetch_all(pool.get_ref())
        .await;

    rows
}

pub async fn find_by_id(pool: web::Data<MySqlPool>, id: i64,) -> Result<Option<MySqlRow>, sqlx::Error> {
    let row = sqlx::query("SELECT id, first_name, last_name, password FROM user WHERE id = ?")
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await;

    row
}

pub async fn save(pool: &MySqlPool, user: &User,) -> Result<u64, sqlx::Error> {
    if !(user.id.is_none()) {
        let result = sqlx::query!(
            r#"
            UPDATE user
            SET first_name = ?, last_name = ?, password = ?
            WHERE id = ?
            "#,
            user.first_name,
            user.last_name,
            user.password,
            user.id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    } else {
        // INSERT
        let result = sqlx::query!(
            r#"
            INSERT INTO user (first_name, last_name, password)
            VALUES (?, ?, ?)
            "#,
            user.first_name,
            user.last_name,
            user.password
        )
        .execute(pool)
        .await?;

        Ok(result.last_insert_id())
    }
}

pub async fn delete(pool: web::Data<MySqlPool>, id: i64,) {
    let _row = sqlx::query("DELETE FROM user WHERE id = ?")
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await;
}