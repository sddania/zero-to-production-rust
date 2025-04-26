use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::types::chrono::Utc;
use uuid::Uuid;

// https://www.joshmcguigan.com/blog/understanding-serde/
#[derive(Deserialize)]
pub struct Info {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<Info>,
    // Retrieving a connection from the application state!
    connection: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
    
}
