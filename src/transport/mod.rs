//!
//! Transport Module
//!
mod base;
mod tangle;
use iota_streams::app_channels::api::tangle::{Address, DefaultF, DefaultTW};

pub trait AsyncTransport: base::AsyncTransport<DefaultTW, DefaultF, Address> {}
impl<T> AsyncTransport for T where T: base::AsyncTransport<DefaultTW, DefaultF, Address> {}

