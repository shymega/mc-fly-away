// SPDX-FileCopyrightText: 2023 Dom Rodriguez (shymega) <shymega+mc-fly-away@shymega.org.uk>
//
// SPDX-License-Identifier: AGPL-3.0-only

pub mod cloud_providers;
pub mod minecraft;
pub mod utils;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
