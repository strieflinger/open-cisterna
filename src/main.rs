// Copyright 2018 The Open Cisterna project developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate config;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate sysfs_gpio;

use rocket::State;
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket_contrib::Json;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

mod maxsonar;

#[derive(Copy, Clone, Debug, Serialize)]
struct Geometry {
    base_area: f64
}

#[derive(Debug)]
struct Range {
    min: f64,
    max: f64
}

#[derive(Debug)]
struct OpenCisternaConfig {
    geometry: Geometry,
    no_detection_distance: usize,
    port: String,
    interval: u64,
    range: Range
}

impl OpenCisternaConfig {
    fn new(cfg: config::Config) -> OpenCisternaConfig {
        let cfg = OpenCisternaConfig {
            geometry: Geometry {
                base_area: cfg.get_float("geometry.base_area").unwrap()
            },
            no_detection_distance: cfg
                .get_float("sensor.no_detection_distance")
                .map(|d| (d * 1000.0) as usize)
                .unwrap(),
            port: cfg.get_str("detection.port").unwrap(),
            interval: match cfg.get_int("detection.interval").unwrap() {
                i if i <= 0 => panic!("detection interval must be strictly positive"),
                i => i as u64
            },
            range: Range {
                min: cfg.get_float("detection.range.min").unwrap(),
                max: cfg.get_float("detection.range.max").unwrap()
            }
        };
        info!("Settings loaded successfully: {:?}", cfg);
        cfg
    }
}

fn read_config() -> OpenCisternaConfig {
    let mut cfg = config::Config::default();
    cfg
        .merge(config::File::with_name("settings")).unwrap()
        .merge(config::Environment::with_prefix("OPEN_CISTERNA")).unwrap();
    OpenCisternaConfig::new(cfg)
}

#[derive(Debug, Serialize)]
struct CisternState {
    level: f64,
    quantity: f64
}

fn detect_distance(port_name: &str) -> Result<u16, String> {
    let distance = maxsonar::read_distance(port_name)?;
    info!("Detected distance: {} mm", distance);
    Ok(distance)
}

fn normalize_distance(r: f64, cfg: &OpenCisternaConfig) -> f64 {
    let nr = r.max(cfg.range.min).min(cfg.range.max);
    if r < cfg.range.min || r > cfg.range.max {
        warn!("Detected distance {} m is out of bounds: {:?}. Normalized to {} m", r, cfg.range, nr);
    }
    nr
}

fn compute_state(distance: f64, cfg: &OpenCisternaConfig) -> CisternState {
    let l = normalize_distance(distance, cfg) - cfg.range.min;
    CisternState {
        level: l / (cfg.range.max - cfg.range.min),
        quantity: l * cfg.geometry.base_area
    }
}

#[get("/cistern/state", format = "application/json")]
fn state(cfg: State<OpenCisternaConfig>, distance: State<Arc<AtomicUsize>>) -> Result<Json<CisternState>, Custom<&'static str>> {
    let d = distance.load(Ordering::Relaxed);
    match d {
        d if d == cfg.no_detection_distance =>
                Err(Custom(
                        Status::InternalServerError,
                        "Fluid surface not detected or too far away")),
        _ => Ok(Json(compute_state((d as f64) / 1000.0, &cfg)))
    }

}

#[get("/cistern/geometry", format = "application/json")]
fn geometry(cfg: State<OpenCisternaConfig>) -> Json<Geometry> {
    Json(cfg.geometry)
}

fn main() {

    // Initialize logging system and read configuration
    env_logger::init();
    let oc_cfg = read_config();

    // Shared state
    let distance = Arc::new(AtomicUsize::new(0));

    // Spawn thread that periodically reads the distance from the sonar range sensor
    let distance_sink = Arc::clone(&distance);
    let interval = oc_cfg.interval;
    let port = oc_cfg.port.clone();
    thread::spawn(move || {
        loop {
            match detect_distance(port.as_str()) {
                Ok(distance) => {
                    distance_sink.store(distance as usize, Ordering::Relaxed);
                    info!("Update current distance to {} mm", distance);
                },
                Err(reason) => error!("Reading distance from sensor failed: {}", reason)
            }
            thread::sleep(Duration::from_secs(interval));
        }
    });

    // Fire up rocket
    rocket::ignite()
        .manage(oc_cfg)
        .manage(Arc::clone(&distance))
        .mount("/", routes![state, geometry])
        .launch();

}
