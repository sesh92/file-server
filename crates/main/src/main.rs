//! Main entrypoint.

use std::convert::Infallible;
use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::Full;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;

/// Responds with Hello World!
async fn hello(_: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}

/// Entrypoint function
#[tokio::main]
pub async fn main() -> Result<(), color_eyre::eyre::Report> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = envfury::must("ADDR")?;

    let listener = TcpListener::bind(addr).await?;
    tracing::info!(message = "Listening on http", ?addr);

    loop {
        let (tcp, _) = listener.accept().await?;
        let io = hyper_util::rt::TokioIo::new(tcp);

        tokio::task::spawn(async move {
            if let Err(error) =
                hyper_util::server::conn::auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                    .serve_connection(io, service_fn(hello))
                    .await
            {
                tracing::error!(message = "Error serving connection", ?error);
            }
        });
    }
}
