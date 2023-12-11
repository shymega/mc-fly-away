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
pub(crate) struct CloudContainerPlan {
    pub(crate) vendor: CloudVendor,
}

#[derive(Debug)]
pub(crate) struct CloudContainerSpec {
    pub(crate) vendor: CloudVendor,
    pub(crate) ram: isize,
    pub(crate) vcpus: isize,
}

#[derive(Debug)]
pub(crate) struct CloudContainerProduct {
    pub(crate) plan: CloudContainerPlan,
    pub(crate) created: bool,
}

#[derive(Debug, Default, Error, PartialEq, Eq)]
pub(crate) enum CloudContainerError {
    #[error("The Provider could not plan the container needed to accept this network request!")] // Include error data.
    CannotPlan,
    #[error("The Provider could not provision. the container needed to accept this network request!")] // Include error data.
    CannotProvision,
    #[default]
    #[error("Unused error.")]
    Noop, // Noop variant.
}

pub(crate) trait CloudContainerProvider {
    fn is_active(&self) -> bool {
        false
    }

    fn is_not_active(&self) -> bool {
        !self.is_active()
    }

    fn is_created(&self) -> bool {
        false
    }

    fn is_destroyed(&self) -> bool {
        !self.is_created()
    }

    fn plan(
        &self,
        spec: CloudContainerSpec,
    ) -> Result<CloudContainerPlan, CloudContainerError> {
        Ok(CloudContainerPlan {
            vendor: spec.vendor,
        })
    }

    fn provision(
        &self,
        plan: CloudContainerPlan,
    ) -> Result<((), CloudContainerProduct), CloudContainerError> {
        Ok((
            (),
            CloudContainerProduct {
                plan,
                created: true,
            },
        ))
    }
}
