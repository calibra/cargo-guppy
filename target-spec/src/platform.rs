// Copyright (c) The cargo-guppy Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use cfg_expr::targets::{get_target_by_triple, TargetInfo};
use std::collections::HashSet;

// This is generated by the build script.
include!(concat!(env!("OUT_DIR"), "/current_platform.rs"));

/// A platform to evaluate target specs against.
#[derive(Clone, Debug)]
pub struct Platform<'a> {
    target_info: &'a TargetInfo,
    target_features: TargetFeatures<'a>,
    flags: HashSet<&'a str>,
}

impl<'a> Platform<'a> {
    /// Creates a new `Platform` from the given triple and target features.
    ///
    /// Returns `None` if this platform wasn't known to `target-spec`.
    pub fn new(triple: impl AsRef<str>, target_features: TargetFeatures<'a>) -> Option<Self> {
        Some(Self {
            target_info: get_target_by_triple(triple.as_ref())?,
            target_features,
            flags: HashSet::new(),
        })
    }

    /// Adds a set of flags to accept.
    ///
    /// A flag is a single token like the `foo` in `cfg(not(foo))`.
    ///
    /// A default `cargo build` will always evaluate flags to false, but custom wrappers may cause
    /// some flags to evaluate to true. For example, as of version 0.6, `cargo web build` will cause
    /// `cargo_web` to evaluate to true.
    pub fn add_flags(&mut self, flags: &[&'a str]) {
        self.flags.extend(flags);
    }

    /// Returns the target triple for this platform.
    pub fn triple(&self) -> &'static str {
        self.target_info.triple
    }

    /// Returns true if this flag was set with `add_flags`.
    pub fn has_flag(&self, flag: impl AsRef<str>) -> bool {
        self.flags.contains(flag.as_ref())
    }

    /// Returns the underlying `TargetInfo`.
    ///
    /// This is not exported since semver compatibility isn't guaranteed.
    pub(crate) fn target_info(&self) -> &'a TargetInfo {
        self.target_info
    }

    /// Returns the set of target features for this platform.
    pub fn target_features(&self) -> &TargetFeatures<'a> {
        &self.target_features
    }
}

impl Platform<'static> {
    /// Returns the current platform, as detected at build time.
    ///
    /// This will return `None` if the current platform was unknown to this version of
    /// `target-spec`.
    pub fn current() -> Option<Self> {
        let target_info = get_target_by_triple(CURRENT_TARGET)?;
        let target_features = TargetFeatures::features(CURRENT_TARGET_FEATURES);
        Some(Self {
            target_info,
            target_features,
            flags: HashSet::new(),
        })
    }
}

/// A set of target features to match.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum TargetFeatures<'a> {
    /// The target features are unknown.
    Unknown,
    /// Only match the specified features.
    Features(HashSet<&'a str>),
}

impl<'a> TargetFeatures<'a> {
    /// Creates a new `TargetFeatures` which matches some features.
    pub fn features(features: &[&'a str]) -> Self {
        TargetFeatures::Features(features.iter().copied().collect())
    }

    /// Creates a new `TargetFeatures` which doesn't match any features.
    pub fn none() -> Self {
        TargetFeatures::Features(HashSet::new())
    }

    /// Returns `Some(true)` if this feature is a match, `Some(false)` if it isn't, and `None` if
    /// the set of target features is unknown.
    pub fn matches(&self, feature: &str) -> Option<bool> {
        match self {
            TargetFeatures::Unknown => None,
            TargetFeatures::Features(features) => Some(features.contains(feature)),
        }
    }
}
