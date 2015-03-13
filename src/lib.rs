#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # iron-try
//!
//! A handler for trying handlers in succession.
//!
//! For any tuple of handlers used in `Try`, `Try`s `Handler`
//! implementation will call each handler's `handle` method until
//! one returns an error-free `Response`.
//!
//! The same procedure is done for `Try<Vec<Box<Handler>>>`.
//!

extern crate iron;

use iron::prelude::*;
use iron::Handler;

/// A handler for trying handlers in succession.
///
/// For any tuple of handlers used in `Try`, `Try`s `Handler`
/// implementation will call each handler's `handle` method until
/// one returns an error-free `Response`.
///
/// The same procedure is done for `Try<Vec<Box<Handler>>>`. Keep
/// in mind the `Vec` must not be empty.
pub struct Try<H>(H);

impl<H: Handler> Handler for Try<H> {
    #[inline]
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.0.handle(req)
    }
}

impl Handler for Try<Vec<Box<Handler>>> {
    #[inline]
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut iter = self.0.iter();
        let first = iter.next()
            .expect("A non-empty Vec of Handlers for Try.")
            .handle(req);

        iter.fold(first, |a, n| a.or_else(|_| n.handle(req)))
    }
}

impl<H1: Handler, H2: Handler> Handler for Try<(H1, H2)> {
    #[inline]
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        (self.0).0.handle(req)
            .or_else(|_| (self.0).1.handle(req))
    }
}

impl<H1, H2, H3> Handler for Try<(H1, H2, H3)>
where H1: Handler, H2: Handler, H3: Handler {
    #[inline]
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        (self.0).0.handle(req)
            .or_else(|_| (self.0).1.handle(req))
            .or_else(|_| (self.0).2.handle(req))
    }
}

impl<H1, H2, H3, H4> Handler for Try<(H1, H2, H3, H4)>
where H1: Handler, H2: Handler, H3: Handler, H4: Handler {
    #[inline]
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        (self.0).0.handle(req)
            .or_else(|_| (self.0).1.handle(req))
            .or_else(|_| (self.0).2.handle(req))
            .or_else(|_| (self.0).3.handle(req))
    }
}

impl<H1, H2, H3, H4, H5> Handler for Try<(H1, H2, H3, H4, H5)>
where H1: Handler, H2: Handler, H3: Handler, H4: Handler,
      H5: Handler {
    #[inline]
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        (self.0).0.handle(req)
            .or_else(|_| (self.0).1.handle(req))
            .or_else(|_| (self.0).2.handle(req))
            .or_else(|_| (self.0).3.handle(req))
            .or_else(|_| (self.0).4.handle(req))
    }
}

impl<H1, H2, H3, H4, H5, H6> Handler for Try<(H1, H2, H3, H4, H5, H6)>
where H1: Handler, H2: Handler, H3: Handler, H4: Handler,
      H5: Handler, H6: Handler {
    #[inline]
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        (self.0).0.handle(req)
            .or_else(|_| (self.0).1.handle(req))
            .or_else(|_| (self.0).2.handle(req))
            .or_else(|_| (self.0).3.handle(req))
            .or_else(|_| (self.0).4.handle(req))
            .or_else(|_| (self.0).5.handle(req))
    }
}

