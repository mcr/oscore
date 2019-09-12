use coap_lite::{error as coap, CoapOption};
use core::fmt;
#[cfg(feature = "std")]
use std::error;

use crate::cbor;

/// The catch-all error type for this module, mostly just wrapping errors from
/// various libraries.
// TODO: Derive PartialEq as soon as cbor does for its error type
#[derive(Debug)]
pub enum Error {
    /// CoAP request doesn't contain OSCORE option.
    NoOscoreOption,
    /// CoAP request doesn't have kid or piv.
    NoKidPiv,
    /// This message has been received already.
    ReplayDetected,
    /// Message contains an unsupported option.
    UnsupportedOption(CoapOption),
    /// Wraps errors from the `cbor` module.
    Cbor(cbor::CborError),
    /// Wraps errors from `hkdf`.
    Hkdf(hkdf::InvalidLength),
    /// Wraps errors from `aes_ccm`.
    Aead(aes_ccm::Error),
    /// Wraps errors from `coap_lite`.
    Coap(coap::MessageError),
}

impl From<cbor::CborError> for Error {
    fn from(e: cbor::CborError) -> Error {
        Error::Cbor(e)
    }
}

impl From<hkdf::InvalidLength> for Error {
    fn from(e: hkdf::InvalidLength) -> Error {
        Error::Hkdf(e)
    }
}

impl From<aes_ccm::Error> for Error {
    fn from(e: aes_ccm::Error) -> Error {
        Error::Aead(e)
    }
}

impl From<coap::MessageError> for Error {
    fn from(e: coap::MessageError) -> Error {
        Error::Coap(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoOscoreOption => {
                write!(f, "CoAP request doesn't contain OSCORE option")
            }
            Error::NoKidPiv => {
                write!(f, "CoAP request doesn't have kid or piv")
            }
            Error::ReplayDetected => {
                write!(f, "This message has been received already")
            }
            Error::UnsupportedOption(o) => {
                write!(f, "Message contains an unsupported option: {:?}", o)
            }
            Error::Cbor(e) => e.fmt(f),
            Error::Hkdf(e) => e.fmt(f),
            Error::Aead(e) => e.fmt(f),
            Error::Coap(e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Cbor(e) => Some(e),
            Error::Hkdf(e) => Some(e),
            Error::Aead(e) => Some(e),
            Error::Coap(e) => Some(e),
            // Other errors that don't implement the Error trait
            _ => None,
        }
    }
}