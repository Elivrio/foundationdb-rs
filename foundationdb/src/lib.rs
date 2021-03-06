// Copyright 2018 foundationdb-rs developers, https://github.com/Clikengo/foundationdb-rs/graphs/contributors
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # FoundationDB Rust Client API
//!
//! This is a wrapper library around the FoundationDB (Fdb) C API. It implements futures based interfaces over the Fdb future C implementations.
//!
//! ## Prerequisites
//!
//! ### Install FoundationDB
//!
//! Install FoundationDB on your system, see [FoundationDB Local Development](https://apple.github.io/foundationdb/local-dev.html), or these instructions:
//!
//! - Ubuntu Linux (this may work on the Linux subsystem for Windows as well)
//!
//! ```console
//! $> curl -O https://www.foundationdb.org/downloads/6.1.12/ubuntu/installers/foundationdb-clients_6.1.12-1_amd64.deb
//! $> curl -O https://www.foundationdb.org/downloads/6.1.12/ubuntu/installers/foundationdb-server_6.1.12-1_amd64.deb
//! $> sudo dpkg -i foundationdb-clients_6.1.12-1_amd64.deb
//! $> sudo dpkg -i foundationdb-server_6.1.12-1_amd64.deb
//! ```
//!
//! - macOS
//!
//! ```console
//! $> curl -O https://www.foundationdb.org/downloads/6.1.12/macOS/installers/FoundationDB-6.1.12.pkg
//! $> sudo installer -pkg FoundationDB-6.1.12.pkg -target /
//! ```
//!
//! - Windows
//!
//! Install [foundationdb-6.1.12-x64.msi](https://www.foundationdb.org/downloads/6.1.12/windows/installers/foundationdb-6.1.12-x64.msi)
//!
//! ## Add dependencies on foundationdb-rs
//!
//! ```toml
//! [dependencies]
//! foundationdb = "0.4"
//! futures = "0.3"
//! ```
//!
//! ## Initialization
//!
//! Due to limitations in the C API, the Client and it's associated Network can only be initialized and run once per the life of a process. Generally the `foundationdb::init` function will be enough to initialize the Client. See `foundationdb::default_api` and `foundationdb::builder` for more configuration options of the Fdb Client.
//!
//! ## Example
//!
//! ```rust
//! use futures::prelude::*;
//!
//! async fn async_main() -> foundationdb::FdbResult<()> {
//!     let db = foundationdb::Database::default()?;
//!
//!     // write a value
//!     let trx = db.create_trx()?;
//!     trx.set(b"hello", b"world"); // errors will be returned in the future result
//!     trx.commit().await?;
//!
//!     // read a value
//!     let trx = db.create_trx()?;
//!     let maybe_value = trx.get(b"hello", false).await?;
//!     let value = maybe_value.unwrap(); // unwrap the option
//!
//!     assert_eq!(b"world", &value.as_ref());
//!
//!     Ok(())
//! }
//!
//! foundationdb::boot(|| {
//!     futures::executor::block_on(async_main()).expect("failed to run");
//! });
//! ```
//!
//! ## API stability
//!
//! *WARNING* Until the 1.0 release of this library, the API may be in constant flux.

#[macro_use]
extern crate static_assertions;

pub mod api;
#[cfg(any(feature = "fdb-5_1", feature = "fdb-5_2", feature = "fdb-6_0"))]
pub mod cluster;
mod database;
mod error;
pub mod future;
mod keyselector;
/// Generated configuration types for use with the various `set_option` functions
#[allow(clippy::all)]
pub mod options;
mod transaction;
pub mod tuple;

#[cfg(any(feature = "fdb-5_1", feature = "fdb-5_2", feature = "fdb-6_0"))]
pub use crate::cluster::Cluster;

pub use crate::database::*;
pub use crate::error::FdbError;
pub use crate::error::FdbResult;
pub use crate::keyselector::*;
pub use crate::transaction::*;

/// Execute `f` with the FoundationDB Client API ready, this can only be called once per process.
///
/// # Examples
///
/// ```rust
/// foundationdb::boot(|| {
///     // do some interesting things with the API...
/// });
/// ```
pub fn boot<T>(f: impl (FnOnce() -> T) + std::panic::UnwindSafe) -> T {
    api::FdbApiBuilder::default()
        .build()
        .expect("foundationdb API to be initialized")
        .boot(f)
        .expect("foundationdb network to be setup")
}

/// Async execute `f` with the FoundationDB Client API ready, this can only be called once per process.
///
/// # Examples
///
/// ```rust
/// foundationdb::boot_async(|| async {
///     // do some interesting things with the API...
/// });
/// ```
pub async fn boot_async<F, Fut, T>(f: F) -> T
where
    F: (FnOnce() -> Fut) + std::panic::UnwindSafe,
    Fut: std::future::Future<Output = T> + std::panic::UnwindSafe,
{
    api::FdbApiBuilder::default()
        .build()
        .expect("foundationdb API to be initialized")
        .boot_async(f)
        .await
        .expect("foundationdb network to be setup")
}

/// Returns the default Fdb cluster configuration file path
#[cfg(target_os = "linux")]
pub fn default_config_path() -> &'static str {
    "/etc/foundationdb/fdb.cluster"
}

/// Returns the default Fdb cluster configuration file path
#[cfg(target_os = "macos")]
pub fn default_config_path() -> &'static str {
    "/usr/local/etc/foundationdb/fdb.cluster"
}

/// Returns the default Fdb cluster configuration file path
#[cfg(target_os = "windows")]
pub fn default_config_path() -> &'static str {
    "C:/ProgramData/foundationdb/fdb.cluster"
}
