// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "fam-wrappers")]
mod fam_wrappers;

// Export 4.20 bindings when the feature kvm-v5_11_0 is not specified.
#[cfg(all(feature = "kvm-v4_20_0", not(feature = "kvm-v5_11_0")))]
#[allow(clippy::all)]
mod bindings_v4_20_0;

// Export 5.11 bindings when kvm-v5_11_0 is specified or no kernel version
// related features are specified.
#[cfg(any(
    feature = "kvm-v5_11_0",
    all(not(feature = "kvm-v4_20_0"), not(feature = "kvm-v5_11_0"))
))]
#[allow(clippy::all)]
mod bindings_v5_11_0;

pub mod bindings {
    #[cfg(any(
        feature = "kvm-v5_11_0",
        all(not(feature = "kvm-v4_20_0"), not(feature = "kvm-v5_11_0"))
    ))]
    pub use super::bindings_v5_11_0::*;

    #[cfg(feature = "fam-wrappers")]
    pub use super::fam_wrappers::*;
}
