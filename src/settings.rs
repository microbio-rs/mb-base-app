// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

// use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

use crate::logging::LogSettings;
use crate::rest::RestSettings;

#[derive(Debug, Deserialize)]
pub struct Settings {
    log: LogSettings,
    rest: RestSettings,
}

impl Settings {
    pub fn rest(&self) -> &RestSettings {
        &self.rest
    }

    pub fn log(&self) -> &LogSettings {
        &self.log
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // let run_mode =
        //     env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("settings"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            // .add_source(
            //     File::with_name(&format!(
            //         "examples/hierarchical-env/config/{}",
            //         run_mode
            //     ))
            //     .required(false),
            // )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            // .add_source(
            //     File::with_name("examples/hierarchical-env/config/local")
            //         .required(false),
            // )
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            .build()?;

        s.try_deserialize()
    }
}
