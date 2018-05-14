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
extern crate rand;
extern crate rocket;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rand::{Rng, thread_rng};
use rocket::State;
use rocket_contrib::Json;

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
struct Config {
    geometry: Geometry,
    range: Range
}

impl Config {
    fn new(cfg: config::Config) -> Config {
        let cfg = Config {
            geometry: Geometry {
                base_area: cfg.get_float("geometry.base_area").unwrap()
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

#[derive(Debug, Serialize)]
struct CisternState {
    level: f64,
    quantity: f64
}

fn detect_range() -> f64 {
    let range = thread_rng().gen_range(0.49, 3.22);
    info!("Detected range: {}", range);
    range
}

fn normalize_range(r: f64, cfg: &Config) -> f64 {
    let nr = r.max(cfg.range.min).min(cfg.range.max);
    if r < cfg.range.min || r > cfg.range.max {
        warn!("Detected range {} is out of bounds: {:?}. Normalized to {}", r, cfg.range, nr);
    }
    nr
}

fn compute_state(range: f64, cfg: &Config) -> CisternState {
    let l = normalize_range(range, cfg) - cfg.range.min;
    CisternState {
        level: l / (cfg.range.max - cfg.range.min),
        quantity: l * cfg.geometry.base_area
    }
}

#[get("/cistern/state", format = "application/json")]
fn state(cfg: State<Config>) -> Json<CisternState> {
    let range = detect_range();
    Json(compute_state(range, &cfg))
}

#[get("/cistern/geometry", format = "application/json")]
fn geometry(cfg: State<Config>) -> Json<Geometry> {
    Json(cfg.geometry)
}

fn main() {
    env_logger::init();
    let mut cfg = config::Config::default();
    cfg
        .merge(config::File::with_name("settings")).unwrap()
        .merge(config::Environment::with_prefix("OPEN_CISTERNA")).unwrap();
    rocket::ignite().manage(Config::new(cfg)).mount("/", routes![state, geometry]).launch();
}
