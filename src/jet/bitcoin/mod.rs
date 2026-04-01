// SPDX-License-Identifier: CC0-1.0

mod environment;

pub use environment::BitcoinEnv;

use crate::bit_encoding::decode;
use crate::BitIter;

use super::init::bitcoin::Bitcoin;
use super::JetEnvironment;
use simplicity_sys::c_jets::frame_ffi::CFrameItem;

impl JetEnvironment for BitcoinEnv {
    type Jet = Bitcoin;
    type CJetEnvironment = ();

    fn c_jet_env(&self) -> &Self::CJetEnvironment {
        &()
    }

    fn c_jet_ptr(
        jet: &Self::Jet,
    ) -> fn(&mut CFrameItem, CFrameItem, &Self::CJetEnvironment) -> bool {
        super::init::bitcoin::c_jet_ptr(jet)
    }

    fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Self::Jet, decode::Error> {
        super::init::bitcoin::decode(bits)
    }
}
