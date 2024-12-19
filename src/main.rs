mod day1;
mod day_2;

use shuttle_poem::ShuttlePoem;
use shuttlings_cch24::main_router;

#[shuttle_runtime::main]
async fn poem(
    #[shuttle_shared_db::Postgres] db: sqlx::PgPool,
) -> ShuttlePoem<impl poem::Endpoint> {
    sqlx::migrate!().run(&db).await.unwrap();
    let app = main_router(db);

    Ok(app.into())
}
