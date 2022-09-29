use std::collections::HashMap;
use tower::{Service, ServiceExt};

// fake webserver that take in a Tower Service called "app", this app is our web application that
// is capable of taking a request and creating a response, this function runs in a loop, generating
// fake requests and get response from the app
pub async fn run<App>(mut app: App)
where
    App: Service<crate::http::Request, Response = crate::http::Response>,
    App::Error: std::fmt::Debug,
    App::Future: Send + 'static,
{
    loop {
        let req = crate::http::Request {
            path_and_query: "/fake/path?page=1".to_owned(),
            headers: HashMap::new(),
            body: Vec::new(),
        };

        let app = match app.ready().await {
            Err(e) => {
                eprintln!("Service not able to accept requests: {:?}", e);
                continue;
            }
            Ok(app) => app,
        };

        let future = app.call(req);
        tokio::spawn(async move {
            match future.await {
                Ok(res) => println!("Successful response: {:?}", res),
                Err(e) => eprintln!("Error occurred: {:?}", e),
            }
        });
    }
}
