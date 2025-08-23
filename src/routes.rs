use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::models::User;
use crate::db::Db;
use crate::cache::Cache;

pub async fn create_user(
    db: web::Data<Db>,
    cache: web::Data<Cache>,
    user: web::Json<User>,
) -> impl Responder {
    let id = db.create_user(&user).await.unwrap();
    cache.set_user(id, &user).await;
    HttpResponse::Ok().json(json!({ "id": id }))
}

pub async fn get_user(
    db: web::Data<Db>,
    cache: web::Data<Cache>,
    id: web::Path<i64>,
) -> impl Responder {
    if let Some(user) = cache.get_user(*id).await {
        return HttpResponse::Ok().json(user);
    }
    if let Some(user) = db.get_user(*id).await.unwrap() {
        cache.set_user(*id, &user).await;
        return HttpResponse::Ok().json(user);
    }
    HttpResponse::NotFound().finish()
}

