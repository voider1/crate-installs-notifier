#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate mac_notification_sys;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
extern crate tokio_core;

use std::mem;
use std::io::{self, Cursor};
use std::thread::sleep;
use std::time::Duration;

use futures::{Future, Stream};
use futures::stream::futures_unordered;
use futures::future::ok;
use mac_notification_sys::send_notification;
use reqwest::unstable::async::{Client, Decoder};
use serde_json::Value;
use tokio_core::reactor::Core;

use config::Config;
use ErrorKind::ReqError;

mod config;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(io::Error);
    }
}

fn main() {
    let mut core = Core::new().expect("Something went wrong with the core");
    let client = Client::new(&core.handle());
    let config = Config::new().unwrap();
    let duration = Duration::new(6, 0);

    let work = futures_unordered(config.crates.iter().map(|c| {
        let uri =
            format!{"https://www.crates.io/api/v1/crates?page=1&per_page=1&q={}", c.name};

        client.get(&uri)
            .send()
            .and_then(|mut res| {
                let body = mem::replace(res.body_mut(), Decoder::empty());
                body.concat2().map_err(Into::into)
            })
            .and_then(|body| {
                let body = Cursor::new(body);
                let v: Value = serde_json::from_reader(body).unwrap();
                ok(v)
            })
    }));

    let notify = work.map_err(ReqError).fold(config, |mut config, b| {
        let name = b["crates"][0]["name"].as_str().unwrap();
        let old_downloads = b["crates"][0]["downloads"].as_i64().unwrap();

        for c in &mut config.crates {
            if c.name == name {
                let difference = old_downloads - c.downloads;
                let message = format!{"Amount of downloads were {} and are now {}. A difference of {}", c.downloads, old_downloads, difference};

                send_notification(name, &None, &message, &None).unwrap();
                c.downloads = old_downloads;
                break;
            }
        }

        let tmp: Result<Config> = Ok(config);
        sleep(duration);
        tmp
    });

    let updated_config = core.run(notify).unwrap();
    updated_config.update().unwrap();
}
