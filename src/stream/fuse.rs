use {Poll, Async};
use stream::Stream;

/// A stream which "fuse"s a stream once it's terminated.
///
/// Normally streams can behave unpredictably when used after they have already
/// finished, but `Fuse` continues to return `None` from `poll` forever when
/// finished.
#[must_use = "streams do nothing unless polled"]
pub struct Fuse<S> {
    stream: S,
    done: bool,
}

// Forwarding impl of Sink from the underlying stream
impl<S> ::sink::Sink for Fuse<S>
    where S: ::sink::Sink
{
    type SinkItem = S::SinkItem;
    type SinkError = S::SinkError;

    fn start_send(&mut self, item: S::SinkItem) -> ::StartSend<S::SinkItem, S::SinkError> {
        self.stream.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), S::SinkError> {
        self.stream.poll_complete()
    }
}

pub fn new<S: Stream>(s: S) -> Fuse<S> {
    Fuse { stream: s, done: false }
}

impl<S: Stream> Stream for Fuse<S> {
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<S::Item>, S::Error> {
        if self.done {
            Ok(Async::Ready(None))
        } else {
            let r = self.stream.poll();
            if let Ok(Async::Ready(None)) = r {
                self.done = true;
            }
            r
        }
    }
}

impl<S> Fuse<S> {
    /// Returns whether the underlying stream has finished or not.
    ///
    /// If this method returns `true`, then all future calls to poll are
    /// guaranteed to return `None`. If this returns `false`, then the
    /// underlying stream is still in use.
    pub fn is_done(&self) -> bool {
        self.done
    }
}
