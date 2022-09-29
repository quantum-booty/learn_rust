mod app;
mod fakeserver;
mod http;

#[tokio::main]
async fn main() {
    crate::fakeserver::run(app::DemoApp::default()).await;
}
