//! # ez-err
//! The goal of this crate is to add simple and easy-to-use error handling. The
//! amount of boilerplate code required to get the full set of features should
//! be minimal. Ez-err includes the stack trace information directly in the error
//! type in case of [`Err`] (almost no overhead if [`Ok`]). The approach is especially
//! useful for any case where the error propagation might be deferred (code storing
//! [`Result`]s in a [`Vec`] and only later checking whether they are [`Ok`] or not).
//!
//! It is also worth taking a look at [`eget`] and [`eget_mut`] for more advanced error
//! messages which include more information about the error.
//!
//! # Use cases
//! This crate can be useful for general purpose error handling. However, it should
//! be helpful in scenarios where no source code information should be contained in
//! the resulting product for any reason. Only relying on ez-err for error handling
//! will provide full power over the error output. Disabling stack trace collection
//! should also remove any source code information from the binary that would have
//! been generated with ez-err.
//!
//! # How to use / Example
//! To use ez-err, you need to add `use ez_err::prelude::*` to your source file.
//! Once that is done, you can use the custom [`Result<T>`] type in your functions
//! and then handle all errors by using `xxx.loc(flc!())?`. It is possible to use this
//! same pattern when converting from any error type to [`EzError`].
//! ```ignore
//! use ez_err::prelude::*;
//! use std::io::Write;
//!
//! fn save_log_output(log: String) -> Result<()> {
//!     // Try to open the file or return an error with stack trace.
//!     let mut file = std::fs::File::open("...").loc(flc!())?;
//!     // Try to write to the file or return an error with stack trace.
//!     write!(&mut file, "{}", log).loc(flc!())?;
//!
//!     // Everything went well without an error.
//!     Ok(())
//! }
//!
//! fn quit(log: String) {
//!     // Print the error to the console if there is any.
//!     let _optional_return: Option<()> = save_log_output(log).handle();
//! }
//! ```
//!
//! # How does it work?
//! `xxx.loc(flc!())?` is made up of 3 parts: the [`loc`] function, the [`flc!`] macro,
//! and the standard `?` operator. The [`flc!`] macro will first expand to a [`ConstLocation`]
//! containing information about the source code location where the macro was invoked.
//! This information is passed into the `loc` function, which will store the location
//! ONLY if it is currently an [`Err`] to minimize the overhead. The standard `?` operator
//! will then perform the already existing logic for error propagation. This approach
//! requires no special backtrace configuration and can produce clean stack traces. It
//! should be very fast by compiling down to just an additional if statement in the [`Ok`]-case.
//!
//! # Why should I use ez-err?
//! The advantage of this crate over others is simplicity. Other error handling
//! crates require manually adding error reasons, which can be helpful but can
//! often be overkill.
//!
//! Here is an example to show the difference between this crate and a popular
//! one ([error-chain](https://crates.io/crates/error-chain)):
//! ```ignore
//! fn error_producer() -> Result<i32> { /* ... */ }
//!
//! // Propagate an error using error-chain
//! fn use_error_chain() -> Result<i32> {
//!     /* ... */
//!     let value: i32 = error_producer().chain_err(|| "error when getting value")?;
//!     Ok(value + 1)
//! }
//!
//! // Propagate an error using ez-err
//! fn use_ez_err() -> ez_err::Result<i32> {
//!     /* ... */
//!     let value: i32 = error_producer().loc(flc!())?;
//!     Ok(value + 1)
//! }
//! ```
//!
//! # Features
//! * `log` - enable compatibility with the [log](https://crates.io/crates/log) crate. The code will by default output to `error!(...)`.
//! * `no_stacktrace` - disable any stacktrace collection. This might be useful in a scenario where leaking source information is problematic.
//!
//! # License
//! This project is licensed under the [MIT license](https://github.com/MariusSoft-LLC/ez-err/blob/main/LICENSE).
//!
//! # Contribution
//! Any contribution intentionally submitted for inclusion in the work by you, shall be licensed as MIT, without any additional terms or conditions.
//!
//! [`loc`]: prelude::LocData::loc
//! [`Result`]: prelude::Result
//! [`Result<T>`]: prelude::Result
//! [`EzError`]: prelude::EzError
//! [`ConstLocation`]: prelude::ConstLocation
//! [`eget`]: prelude::SliceExt::eget
//! [`eget_mut`]: prelude::SliceExtMut::eget_mut

#![warn(missing_docs)]
#![deny(warnings)]

pub mod core;
pub mod prelude;
pub mod slice_ext;
