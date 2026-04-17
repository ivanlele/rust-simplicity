// SPDX-License-Identifier: CC0-1.0

use super::init::core::Core;
use super::JetEnvironment;

/// Type alias for the Core jet environment.
#[derive(Default, Debug)]
pub struct CoreEnv {
    _inner: (),
}

impl CoreEnv {
    pub fn new() -> Self {
        Self { _inner: () }
    }
}

impl JetEnvironment for CoreEnv {
    type Jet = Core;

    fn c_jet_env(&self) -> &<Self::Jet as super::Jet>::CJetEnvironment {
        &()
    }
}
