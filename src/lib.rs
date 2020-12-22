//! A driver for the KSZ8863 Ethernet Switch.
//!
//! This driver is split into two main modules:
//!
//! - [`miim`](./miim/index.html) for the MII Management Interface (MIIM).
//! - [`smi`](./smi/index.html) for the Serial Management Interface (SMI).
//!
//! While these two terms often refer to same protocol, their usage in the KSZ8863 documentation
//! refers to two distinct protocols and sets of registers. Please refer to the datasheet for
//! details.
//!
//! These modules contain a type and module for every documented register along with typed access
//! to each of their respective fields. High-level read/write/modify access to these registers are
//! provided via the `Miim` and `Smi` types respectively.
//!
//! *Note that the SPI and I2C interfaces are not currently supported, though PRs are welcome.*
//!
//! # Usage
//!
//! At the foundation of this crate are the `miim::{Read, Write}` and `smi::{Read, Write}` traits.
//! The first step is to implement these for your respective MIIM and SMI interfaces. For details
//! on how to implement these, visit sections `3.3.10` and `3.3.11` of the datasheet.
//!
//! Implementing these traits unlocks high level access via the [`Smi`](./smi/struct.Smi.html) and
//! [`Miim`](./miim/struct.Miim.html) interface type wrappers. These types provide short-hand
//! methods for reading, writing and modifying registers and their individual fields. The provided
//! API for these types is inspired by the `svd2rust` crate.
//!
//! Here is an example of using the `Miim`.
//!
//! ```rust
//! use ksz8863::{miim, Miim};
//!
//! fn main() {
//! #    let miim_iface = miim::Map::default();
//!     // Wrap the type that impls the Read/Write traits with `Miim`.
//!     // Note: We could also wrap `&mut miim_iface` here if we only local scope access is needed.
//!     let mut miim = Miim(miim_iface);
//!
//!     // Specify which phy we want to communicate with via its PHY address.
//!     let mut phy = miim.phy(0);
//!
//!     // Read the value of the "Basic Control Register".
//!     assert_eq!(phy.bcr().read().unwrap(), miim::Bcr::default());
//!
//!     // Modify the "Force 100BT" field of the "Basic Control Register" in place.
//!     let mut bcr = phy.bcr();
//!     assert!(bcr.read().unwrap().read().force_fd().bit_is_clear());
//!     bcr.modify(|w| w.force_fd().set_bit()).unwrap();
//!     let reg = bcr.read().unwrap();
//!     assert!(reg != miim::Bcr::default());
//!     assert!(reg.read().force_fd().bit_is_set());
//! }
//! ```
//!
//! The `Smi` API is similar, but we don't need to specify a PHY address.
//!
//! ```rust
//! use ksz8863::{smi, Smi};
//!
//! fn main() {
//! #    let smi_iface = smi::Map::default();
//!     let mut smi = Smi(smi_iface);
//!     assert_eq!(smi.gc1().read().unwrap(), smi::Gc1::default());
//!     smi.gc1().modify(|w| w.tx_flow_control().clear_bit()).unwrap();
//!     assert!(smi.gc1().read().unwrap() != smi::Gc1::default());
//! }
//! ```
//!
//! ## Extras
//!
//! The `Address` type in each module represents the unique index at which the register is located.
//!
//! The `State` type from each module is a dynamic representation of register state, useful for
//! storing the state of multiple registers in a collection.
//!
//! The `Map` type from each module is a collection that is guaranteed to contain the state of all
//! registers. This is useful for remotely monitoring the state of registers while reducing I/O,
//! and for simulating an MIIM/SMI interface in the case that you don't have access to one.
//!
//! # Features
//!
//! - `hash-32` provides `Hash32` implementations from the `hash32` crate.
//! - `serde` provides `Deserialize` and `Serialize` implementations.
//! - `ufmt` provides `ufmt::uDebug` implementations.
//!
//! All of these features are **opt-in** and disabled by default.

#![no_std]
#![recursion_limit = "256"]

#[macro_use]
mod macros;
pub mod miim;
pub mod smi;

pub use miim::Miim;
pub use smi::Smi;

/// The error returned when no attempting to produce an `Address` from an unknown byte.
#[derive(Debug)]
pub struct InvalidAddress;

/// Allow for using bit values (1 and 0) as default values in register macro.
trait IntoBool {
    /// Convert `self` into a `bool`.
    fn into_bool(self) -> bool;
}

impl IntoBool for bool {
    fn into_bool(self) -> bool {
        self
    }
}

impl IntoBool for u8 {
    fn into_bool(self) -> bool {
        self != 0
    }
}
