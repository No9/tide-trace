#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.middleware(tide_trace::USDTMiddleware::new());
    app.at("/route1").get(|_| async { Ok("Route 1") });
    app.at("/route2").get(|_| async { Ok("Route 2") });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
