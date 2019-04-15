#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! Async readiness traits. Useful when implementing async state machines
//! that can later be wrapped in dedicated futures.
//!
//! ## Example
//!
//! ```rust
//! #![feature(futures_api)]
//!
//! use std::pin::Pin;
//! use std::task::{Context, Poll};
//! use futures::prelude::*;
//! use async_ready::AsyncReady;
//! use std::io;
//!
//! struct Fut;
//!
//! impl Future for Fut {
//!   type Output = ();
//!   fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
//!     Poll::Ready(())
//!   }
//! }
//!
//! impl AsyncReady for Fut {
//!   type Ok = ();
//!   type Err = io::Error;
//!
//!   fn poll_ready(
//!     mut self: Pin<&mut Self>,
//!     cx: &mut Context<'_>,
//!   ) -> Poll<Result<Self::Ok, Self::Err>> {
//!     Poll::Ready(Ok(()))
//!   }
//! }
//! ```

#![feature(futures_api)]

use std::pin::Pin;
use std::task::{Context, Poll};

/// Determine if the underlying API can be written to.
pub trait AsyncWriteReady {
  /// The type of successful values yielded by this trait.
  type Ok;

  /// The type of failures yielded by this trait.
  type Err: std::error::Error + Send + Sync;

  /// Check if the underlying API can be written to.
  fn poll_write_ready(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Result<Self::Ok, Self::Err>>;
}

/// Determine if the underlying API can be read from.
pub trait AsyncReadReady {
  /// The type of successful values yielded by this trait.
  type Ok;

  /// The type of failures yielded by this trait.
  type Err: std::error::Error + Send + Sync;

  /// Check if the underlying API can be read from.
  fn poll_read_ready(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Result<Self::Ok, Self::Err>>;
}

/// Determine if a struct is async-ready to yield futures.
///
/// This is useful when a `Stream` borrows an internal struct, and the internal
/// struct is in charge of establishing the io channel. That way the stream and
/// the readiness can be decoupled.
///
/// Once the IO channel is async-ready, `poll_async-ready` should always return
/// `Poll::Ready`.
pub trait AsyncReady {
  /// The type of successful values yielded by this trait.
  type Ok;

  /// The type of failures yielded by this trait.
  type Err: std::error::Error + Send + Sync;

  /// Check if the stream can be read from.
  fn poll_ready(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Result<Self::Ok, Self::Err>>;
}
