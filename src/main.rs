// SPDX-FileCopyrightText: 2023 Dom Rodriguez (shymega) <shymega+mc-fly-away@shymega.org.uk>
//
// SPDX-License-Identifier: AGPL-3.0-only

pub(crate) mod cloud_providers;
pub(crate) mod minecraft;
pub(crate) mod utils;

#[tokio::main]
async fn main() -> Result<(), std::error::Error> {
    Ok(())
}
