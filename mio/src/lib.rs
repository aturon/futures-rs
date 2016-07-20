//! A binding to mio giving it a future/stream interface on top.
//!
//! This library contains the rudimentary bindings to an event loop in mio which
//! provides future and stream-based abstractions of all the underlying I/O
//! objects that mio provides internally.
//!
//! Currently very much a work in progress, and breakage should be expected!

#![deny(missing_docs)]

extern crate mio;
extern crate futures;
extern crate slab;

#[macro_use]
extern crate scoped_tls;

#[macro_use]
extern crate log;

use std::io;
use futures::Future;
use futures::stream::Stream;

mod readiness_stream;
mod event_loop;
mod tcp;
mod buf_reader;
mod buf_writer;
#[path = "../../src/slot.rs"]
mod slot;
#[path = "../../src/lock.rs"]
mod lock;

/// A convenience typedef around a `Future` whose error component is `io::Error`
pub type IoFuture<T> = Future<Item=T, Error=io::Error>;

/// A convenience typedef around a `Stream` whose error component is `io::Error`
pub type IoStream<T> = Stream<Item=T, Error=io::Error>;

pub use event_loop::{Loop, LoopHandle, Direction};
pub use readiness_stream::{ReadinessStream, ReadinessPair};
pub use tcp::{TcpListener, TcpStream, TcpSource};
pub use buf_reader::{BufReader, InputBuf};
pub use buf_writer::{BufWriter, Flush, Reserve};
