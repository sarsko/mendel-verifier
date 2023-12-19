// © 2022, ETH Zurich
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Encoders of pure functions.

mod interface;
mod encoder_high;
mod encoder_poly;

pub(crate) use encoder_poly::FunctionCallInfo;
pub(crate) use interface::{
    PureEncodingContext, PureFunctionEncoderInterface, PureFunctionEncoderState,
};
