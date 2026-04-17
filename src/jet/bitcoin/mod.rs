// SPDX-License-Identifier: CC0-1.0

mod environment;

pub use environment::BitcoinEnv;

use super::init::bitcoin::Bitcoin;
use super::JetEnvironment;

impl JetEnvironment for BitcoinEnv {
    type Jet = Bitcoin;

    fn c_jet_env(&self) -> &<Self::Jet as super::Jet>::CJetEnvironment {
        &()
    }
}
