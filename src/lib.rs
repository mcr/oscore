//! An experimental
//! [OSCORE](https://tools.ietf.org/html/rfc8613)
//! implementation with
//! [EDHOC](https://tools.ietf.org/html/draft-selander-ace-cose-ecdhe-14)
//! key exchange, intended for embedded devices.
//!
//! The EDHOC implementation is based on
//! [draft 14](https://tools.ietf.org/html/draft-selander-ace-cose-ecdhe-14)
//! of the EDHOC specification. It only does authentication with raw public
//! keys (RPK), so it covers the asymmetric authentication scenario, but not
//! the symmetric one using pre-shared keys (PSK).
//! On the OSCORE side, it does key derivation using the master secret and
//! master salt, which can be established with EDHOC.
//!
//! There is [documentation](https://martindisch.github.io/oscore/oscore/) as
//! well as a [demo implementation](https://github.com/martindisch/oscore-demo)
//! using this library, with a resource server on an STM32F3, a client on an
//! STM32F4 and a CoAP proxy running on a Raspberry Pi.
//!
//! ## Security
//! This should **not currently be used in production code**, use at your own
//! risk.

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

mod cbor;

pub mod edhoc;
pub mod oscore;
