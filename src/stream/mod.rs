#![allow(missing_docs)]

use std::sync::Arc;

use {Wake, Tokens, IntoFuture};

mod channel;
mod iter;
pub use self::channel::{channel, Sender, Receiver};
pub use self::iter::{iter, IterStream};

mod and_then;
mod collect;
mod filter;
mod filter_map;
mod flat_map;
mod fold;
mod for_each;
mod fuse;
mod future;
mod map;
mod map_err;
mod or_else;
mod skip_while;
mod then;
pub use self::and_then::AndThen;
pub use self::collect::Collect;
pub use self::filter::Filter;
pub use self::filter_map::FilterMap;
pub use self::flat_map::FlatMap;
pub use self::fold::Fold;
pub use self::for_each::ForEach;
pub use self::fuse::Fuse;
pub use self::future::StreamFuture;
pub use self::map::Map;
pub use self::map_err::MapErr;
pub use self::or_else::OrElse;
pub use self::skip_while::SkipWhile;
pub use self::then::Then;

mod impls;

pub type StreamResult<T, E> = Result<Option<T>, E>;

pub trait Stream: Send + 'static {
    type Item: Send + 'static;
    type Error: Send + 'static;

    fn poll(&mut self, tokens: &Tokens)
            -> Option<StreamResult<Self::Item, Self::Error>>;

    fn schedule(&mut self, wake: Arc<Wake>);

    fn boxed(self) -> Box<Stream<Item=Self::Item, Error=Self::Error>>
        where Self: Sized
    {
        Box::new(self)
    }

    fn into_future(self) -> StreamFuture<Self>
        where Self: Sized
    {
        future::new(self)
    }

    fn map<U, F>(self, f: F) -> Map<Self, F>
        where F: FnMut(Self::Item) -> U + Send + 'static,
              U: Send + 'static,
              Self: Sized
    {
        map::new(self, f)
    }

    fn map_err<U, F>(self, f: F) -> MapErr<Self, F>
        where F: FnMut(Self::Error) -> U + Send + 'static,
              U: Send + 'static,
              Self: Sized
    {
        map_err::new(self, f)
    }

    fn filter<F>(self, f: F) -> Filter<Self, F>
        where F: FnMut(&Self::Item) -> bool + Send + 'static,
              Self: Sized
    {
        filter::new(self, f)
    }

    fn filter_map<F, B>(self, f: F) -> FilterMap<Self, F>
        where F: FnMut(Self::Item) -> Option<B> + Send + 'static,
              Self: Sized
    {
        filter_map::new(self, f)
    }

    fn then<F, U>(self, f: F) -> Then<Self, F, U>
        where F: FnMut(Result<Self::Item, Self::Error>) -> U + Send + 'static,
              U: IntoFuture,
              Self: Sized
    {
        then::new(self, f)
    }

    fn and_then<F, U>(self, f: F) -> AndThen<Self, F, U>
        where F: FnMut(Self::Item) -> U + Send + 'static,
              U: IntoFuture<Error=Self::Error>,
              Self: Sized
    {
        and_then::new(self, f)
    }

    fn or_else<F, U>(self, f: F) -> OrElse<Self, F, U>
        where F: FnMut(Self::Error) -> U + Send + 'static,
              U: IntoFuture<Item=Self::Item>,
              Self: Sized
    {
        or_else::new(self, f)
    }

    fn collect(self) -> Collect<Self> where Self: Sized {
        collect::new(self)
    }

    fn fold<F, T>(self, init: T, f: F) -> Fold<Self, F, T>
        where F: FnMut(T, Self::Item) -> T + Send + 'static,
              T: Send + 'static,
              Self: Sized
    {
        fold::new(self, f, init)
    }

    // fn flatten(self) -> Flatten<Self>
    //     where Self::Item: IntoFuture,
    //           <<Self as Stream>::Item as IntoFuture>::Error:
    //                 From<<Self as Stream>::Error>,
    //           Self: Sized
    // {
    //     Flatten {
    //         stream: self,
    //         future: None,
    //     }
    // }

    fn flat_map(self) -> FlatMap<Self>
        where Self::Item: Stream,
              <Self::Item as Stream>::Error: From<Self::Error>,
              Self: Sized
    {
        flat_map::new(self)
    }

    // TODO: should this closure return a Result?
    fn skip_while<P>(self, pred: P) -> SkipWhile<Self, P>
        where P: FnMut(&Self::Item) -> Result<bool, Self::Error> + Send + 'static,
              Self: Sized,
    {
        skip_while::new(self, pred)
    }

    // TODO: should this closure return a result?
    fn for_each<F>(self, f: F) -> ForEach<Self, F>
        where F: FnMut(Self::Item) -> Result<(), Self::Error> + Send + 'static,
              Self: Sized,
    {
        for_each::new(self, f)
    }

    /// Fuse a stream such that `poll`/`schedule` will never again be called
    /// once it has terminated (signaled emptyness or an error).
    ///
    /// Currently once a stream has returned `Some(Ok(None))` from `poll` any further
    /// calls could exhibit bad behavior such as block forever, panic, never
    /// return, etc. If it is known that `poll` may be called too often then
    /// this method can be used to ensure that it has defined semantics.
    ///
    /// Once a stream has been `fuse`d and it terminates, then
    /// it will forever return `None` from `poll` again (never resolve). This,
    /// unlike the trait's `poll` method, is guaranteed.
    ///
    /// Additionally, once a stream has completed, this `Fuse` combinator will
    /// never call `schedule` on the underlying stream.
    fn fuse(self) -> Fuse<Self> where Self: Sized {
        fuse::new(self)
    }
}
