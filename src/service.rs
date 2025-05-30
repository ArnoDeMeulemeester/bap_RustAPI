use crate::repository;
use actix_web::{web, Responder, HttpResponse};
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};
use crate::models::user::User;

pub async fn get_all_users(pool: web::Data<MySqlPool>) -> impl Responder {
  let rows = repository::find_all(pool).await;

    match rows {
        Ok(results) => {
            let users: Vec<User> = results
                .into_iter()
                .map(|row: MySqlRow| User {
                    id: row.get("id"),
                    first_name: row.get("first_name"),
                    last_name: row.get("last_name"),
                    password: row.get("password")
                })
                .collect(); 
            return HttpResponse::Ok().json(users)
        }
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            return HttpResponse::InternalServerError().body("Database error")
        }
    }
}

pub async fn get_user_by_id(path: web::Path<u32>, pool: web::Data<MySqlPool>) -> impl Responder {
  let id = path.into_inner() as i64;
  match repository::find_by_id(pool, id).await {
    Ok(Some(row)) => {
        let user = User {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            password: row.get("password"),
        };
        HttpResponse::Ok().json(user)
    }
    Ok(None) => HttpResponse::NotFound().body("User not found"),
    Err(err) => {
        eprintln!("DB error: {:?}", err);
        HttpResponse::InternalServerError().body("Database error")
    }
  }
}

pub async fn save_user(user: web::Json<User>, pool: web::Data<MySqlPool>) -> impl Responder {
  let mut user = user.into_inner();
  match repository::save(pool.get_ref(), &user).await {
    Ok(result) => {
      user.id = Some(result as i64);
      return HttpResponse::Ok().json(user);
    },
    Err(err) => {
        eprintln!("DB error: {:?}", err);
        HttpResponse::InternalServerError().body(format!("Failed to save user: {}", err))
    }
  }
}

pub async fn update_user(path: web::Path<u32>, user: web::Json<User>, pool: web::Data<MySqlPool>) -> impl Responder {
  let id = path.into_inner() as i64;
  let mut user = user.into_inner();
  user.id = Some(id);
  match repository::save(pool.get_ref(), &user).await {
      Ok(_result) => HttpResponse::Ok().json(user),
      Err(err) => {
          eprintln!("DB error: {:?}", err);
          HttpResponse::InternalServerError().body("Failed to save user")
      }
  }
}

pub async fn delete_user(path: web::Path<u32>, pool: web::Data<MySqlPool>) -> impl Responder{
  let id = path.into_inner() as i64;
  repository::delete(pool, id).await;
  return HttpResponse::Ok().body("User deleted");
}