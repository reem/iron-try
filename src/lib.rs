#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # iron-try
//!
//! A handler for chaining sub-handlers.
//!

extern crate iron;

pub struct Try<H: tryhandler::TryHandler>(H);

mod tryhandler {
    use iron::Handler;
    use iron::prelude::*;

    use std::iter::IntoIterator;

    pub trait TryHandler: Send + Sync {
        fn try_handle(&self, req: &mut Request) -> IronResult<Response>;
    }

    impl<'a, I: IntoIterator<Item=&'a Handler> + Send + Sync> TryHandler for I {
        /// ## Panics
        ///
        /// Panics if self yields an empty iterator.
        fn try_handle(&self, req: &mut Request) -> IronResult<Response> {
            let mut iter = self.into_iter();
            let first = iter.next()
                .expect("An iterator with at least on element.")
                .handle(req);

            iter.fold(first, |a, n| a.or_else(|_| n.handle(req)))
        }
    }

    impl<H: Handler> TryHandler for H {
        fn try_handle(&self, req: &mut Request) -> IronResult<Response> {
            self.handle(req)
        }
    }

    impl<H1: Handler, H2: Handler> TryHandler for (H1, H2) {
        fn try_handle(&self, req: &mut Request) -> IronResult<Response> {
            self.0.handle(req)
                .or_else(move |_| self.1.handle(req))
        }
    }

    impl<H1, H2, H3> TryHandler for (H1, H2, H3)
    where H1: Handler, H2: Handler, H3: Handler {
        fn try_handle(&self, req: &mut Request) -> IronResult<Response> {
            self.0.handle(req)
                .or_else(|_| self.1.handle(req))
                .or_else(|_| self.2.handle(req))
        }
    }

    impl<H1, H2, H3, H4> TryHandler for (H1, H2, H3, H4)
    where H1: Handler, H2: Handler, H3: Handler, H4: Handler {
        fn try_handle(&self, req: &mut Request) -> IronResult<Response> {
            self.0.handle(req)
                .or_else(|_| self.1.handle(req))
                .or_else(|_| self.2.handle(req))
                .or_else(|_| self.3.handle(req))
        }
    }

    impl<H1, H2, H3, H4, H5> TryHandler for (H1, H2, H3, H4, H5)
    where H1: Handler, H2: Handler, H3: Handler, H4: Handler, H5: Handler {
        fn try_handle(&self, req: &mut Request) -> IronResult<Response> {
            self.0.handle(req)
                .or_else(|_| self.1.handle(req))
                .or_else(|_| self.2.handle(req))
                .or_else(|_| self.3.handle(req))
                .or_else(|_| self.4.handle(req))
        }
    }

    impl<H1, H2, H3, H4, H5, H6> TryHandler for (H1, H2, H3, H4, H5, H6)
    where H1: Handler, H2: Handler, H3: Handler, H4: Handler,
          H5: Handler, H6: Handler {
        fn try_handle(&self, req: &mut Request) -> IronResult<Response> {
            self.0.handle(req)
                .or_else(|_| self.1.handle(req))
                .or_else(|_| self.2.handle(req))
                .or_else(|_| self.3.handle(req))
                .or_else(|_| self.4.handle(req))
                .or_else(|_| self.5.handle(req))
        }
    }

    impl<H1, H2, H3, H4, H5, H6, H7> TryHandler for (H1, H2, H3, H4, H5, H6, H7)
    where H1: Handler, H2: Handler, H3: Handler, H4: Handler,
          H5: Handler, H6: Handler, H7: Handler {
        fn try_handle(&self, req: &mut Request) -> IronResult<Response> {
            self.0.handle(req)
                .or_else(|_| self.1.handle(req))
                .or_else(|_| self.2.handle(req))
                .or_else(|_| self.3.handle(req))
                .or_else(|_| self.4.handle(req))
                .or_else(|_| self.5.handle(req))
                .or_else(|_| self.6.handle(req))
        }
    }

    impl<H1, H2, H3, H4, H5, H6, H7, H8>
    TryHandler for (H1, H2, H3, H4, H5, H6, H7, H8)
    where H1: Handler, H2: Handler, H3: Handler, H4: Handler,
          H5: Handler, H6: Handler, H7: Handler, H8: Handler {
        fn try_handle(&self, req: &mut Request) -> IronResult<Response> {
            self.0.handle(req)
                .or_else(|_| self.1.handle(req))
                .or_else(|_| self.2.handle(req))
                .or_else(|_| self.3.handle(req))
                .or_else(|_| self.4.handle(req))
                .or_else(|_| self.5.handle(req))
                .or_else(|_| self.6.handle(req))
                .or_else(|_| self.7.handle(req))
        }
    }
}

