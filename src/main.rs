mod day1;
mod day_2;

use shuttle_poem::ShuttlePoem;
use shuttlings_cch24::main_router;

#[shuttle_runtime::main]
async fn poem() -> ShuttlePoem<impl poem::Endpoint> {
    let app = main_router();

    Ok(app.into())
}
