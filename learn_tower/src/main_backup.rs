use std::sync::{atomic::AtomicUsize, Arc};

// mod app;
mod fakeserver;
mod http;
mod util;

#[tokio::main]
async fn main() {
    // crate::fakeserver::run(app::DemoApp::default()).await;
    let counter = Arc::new(AtomicUsize::new(0));
    crate::fakeserver::run(util::app_fn(move |mut req| {
        let counter = counter.clone();
        async move {
            println!("Handling a request for {}", req.path_and_query);
            let counter = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            anyhow::ensure!(counter % 4 != 2, "Failing 25% of the time, just for fun");
            req.headers
                .insert("X-Counter".to_owned(), counter.to_string());
            let res = crate::http::Response {
                status: 200,
                headers: req.headers,
                body: req.body,
            };
            Ok::<_, anyhow::Error>(res)
        }
    }))
    .await;
}
