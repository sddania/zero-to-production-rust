use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::{types::chrono::Utc, PgPool};
use tracing::Instrument;
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
    // Let's generate a random unique identifier
    let request_id = Uuid::new_v4();
    // Spans, like logs, have an associated level
    // `info_span` creates a span at the info-level
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name= %form.name
    );
    // Using `enter` in an async function is a recipe for disaster!
    // Bear with me for now, but don't do this at home.
    // See the following section on `Instrumenting Futures`
    let _request_span_guard = request_span.enter();

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
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
    // First we attach the instrumentation, then we `.await` it
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

    // `_request_span_guard` is dropped at the end of `subscribe`
    // That's when we "exit" the span
}
