// SPDX-License-Identifier: CC0-1.0

mod c_env;
mod environment;
#[cfg(test)]
mod tests;

pub use environment::{ElementsEnv, ElementsUtxo};

use super::init::elements::Elements;
use super::JetEnvironment;

/// Type alias for the Elements transaction environment.
pub type ElementsTxEnv = ElementsEnv<std::sync::Arc<elements::Transaction>>;

impl JetEnvironment for ElementsTxEnv {
    type Jet = Elements;

    fn c_jet_env(&self) -> &<Self::Jet as super::Jet>::CJetEnvironment {
        self.c_tx_env()
    }
}
