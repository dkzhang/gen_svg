mod config;
mod element;
mod gen_element;
mod parse;
mod shape;
mod create_svg;
mod create_svg_metering;
mod get_projects;
mod my_utils;


use crate::shape::Draw;
use svg::Node;

use serde_json;
use simplelog::*;
use std::fs::File;

use crate::config::{AppConfig};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

use axum::extract::Query;
use axum::http::header;
use axum::{
    body::Body,
    http::StatusCode,
    response::IntoResponse,
    response::Response,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Deserializer, Serialize};
use tokio::signal;
use crate::create_svg::create_svg;
use crate::create_svg_metering::create_svg_metering;
use crate::my_utils::date::{DateInt, int_to_date70};

fn load_config_style<P: AsRef<Path>>(path: P) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: AppConfig = toml::from_str(&contents)?;
    Ok(config)
}

#[tokio::main]
async fn main() {
    // initialize tracing
    // tracing_subscriber::fmt::init();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/svg", get(get_svg))
        .route("/svgm", get(get_svg_metering));

    tracing::info!("Listening on 0.0.0.0:8080");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}

async fn root() -> impl IntoResponse {
    Response::new(Body::from("Hello, World!"))
}

async fn get_svg(Query(dl): Query<DateDateLoc>) -> impl IntoResponse {
    if !dl.is_valid() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Invalid parameters"))
            .unwrap();
    }

    let svg = create_svg(&dl);

    let mut response = Response::new(Body::from(svg));

    response.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/svg+xml"),
    );

    return response;
}

async fn get_svg_metering(Query(dl): Query<DateDateLoc>) -> impl IntoResponse {
    if !dl.is_valid() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Invalid parameters"))
            .unwrap();
    }

    let svg = create_svg_metering(&dl);

    let mut response = Response::new(Body::from(svg));

    response.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/svg+xml"),
    );

    return response;
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DateDateLoc {
    #[serde(default)]
    start_date: DateInt,
    end_date: i32,
    location: i32,
}

impl DateDateLoc {
    fn is_valid(&self) -> bool {
        let start_date = int_to_date70(self.start_date);
        let end_date = int_to_date70(self.end_date);

        if start_date.is_none() || end_date.is_none() {
            return false;
        }

        if start_date.unwrap() > end_date.unwrap() {
            return false;
        }

        if self.location < 1 || self.location > 7 {
            return false;
        }

        return true;
    }
}







// http://127.0.0.1:8080/svg?start_date=20230701&end_date=20231010&location=1
// http://120.53.227.136:10830/svg?start_date=20230701&end_date=20231010&location=1
