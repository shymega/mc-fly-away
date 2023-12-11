// SPDX-FileCopyrightText: 2023 Dom Rodriguez (shymega) <shymega+mc-fly-away@shymega.org.uk>
//
// SPDX-License-Identifier: AGPL-3.0-only

use thiserror::Error;
use anyhow::Result;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum CloudVendor {
    #[cfg(feature = "fly-io")]
    Fly,
    #[cfg(feature = "aws")]
    Aws,
    #[cfg(feature = "azure")]
    Azure,
    #[cfg(feature = "gcp")]
    Gcp,
    #[default]
    Undefined,
}

#[derive(Debug)]
pub(crate) enum CloudContainerState {
    Created(CloudContainerSpec),
    Err(CloudContainerError),
    Active(CloudContainerSpec),
    Inactive(CloudContainerSpec),
}

#[derive(Debug)]
pub(crate) struct CloudContainerSpec {
    pub(crate) vendor: CloudVendor,
    pub(crate) ram: i32,
    pub(crate) vcpus: i32,
}

#[derive(Debug, Default, Error, PartialEq, Eq)]
pub(crate) enum CloudContainerError {
    #[error("The Provider could not plan the container needed to accept this network request!")] // Include error data.
    CannotPlan,
    #[error("The Provider could not provision the container needed to accept this network request!")] // Include error data.
    CannotProvision,
    #[default]
    #[error("Unused error.")]
    Noop, // Noop variant.
}

pub(crate) trait CloudContainerProvider {
    type Error = CloudContainerError;

    fn active(&self) -> bool {
        false
    }

    fn inactive(&self) -> bool {
        !self.is_active()
    }

    fn created(&self) -> bool {
        false
    }

    fn destroyed(&self) -> bool {
        !self.is_created()
    }

    fn provision(
        &self,
        plan: CloudContainerPlan,
    ) -> Result<CloudContainerState, Error>;
}
