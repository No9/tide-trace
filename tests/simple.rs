mod test_utils;
use async_std::prelude::*;
use async_std::task;
use std::time::Duration;

// use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response, StatusCode};

#[test]
fn hello_world() -> Result<(), http_types::Error> {
    task::block_on(async {
        let port = test_utils::find_port().await;
        let server = task::spawn(async move {
            let mut app = tide::new();
            app.middleware(USDTMiddleware::new(0));
            app.at("/").get(move |mut req: Request<()>| async move {
                assert_eq!(req.body_string().await.unwrap(), "nori".to_string());
                assert!(req.local_addr().unwrap().contains(&port.to_string()));
                assert!(req.peer_addr().is_some());
                let mut res = Response::new(StatusCode::Ok);
                res.set_body("says hello");
                Ok(res)
            });
            app.listen(("localhost", port)).await?;
            Result::<(), http_types::Error>::Ok(())
        });

        let client = task::spawn(async move {
            task::sleep(Duration::from_millis(100)).await;
            let string = surf::get(format!("http://localhost:{}", port))
                .body_string("nori".to_string())
                .recv_string()
                .await
                .unwrap();
            assert_eq!(string, "says hello".to_string());
            Ok(())
        });

        server.race(client).await
    })
}