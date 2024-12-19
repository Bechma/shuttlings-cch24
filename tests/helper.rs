pub fn main_router() -> impl poem::Endpoint {
    shuttlings_cch24::main_router(sqlx::Pool::connect_lazy("sqlite://memory").unwrap())
}