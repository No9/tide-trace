#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.middleware(tide_trace::USDTMiddleware::new());
    app.at("/").get(|_| async {
        let retval = "Hello, world!";
        tide_trace::probe("tag-text".to_string(), retval.to_string());
        Ok("Hello, world!") 
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}