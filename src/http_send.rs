use reqwest::{self, Client, Request, RequestBuilder};
use std::io::{self, Read};
use std::fmt::Debug;
use std::ops::Deref;
use Result;

/// Abstracts away the process of turning an HTTP request into an HTTP response
pub trait HttpSend: Debug + Clone {
    type Output: Read + Debug;

    /// Converts an HTTP request into an HTTP response
    fn execute(&self, client: &Client, request: Request) -> Result<Response<Self::Output>>;

    /// Convenience method so that .build() doesn't have to be called at every
    /// call site
    fn send(&self, client: &Client, builder: RequestBuilder) -> Result<Response<Self::Output>> {
        let request = builder.build()?;
        self.execute(client, request)
    }
}

/// Wrapper type for responses
#[derive(Debug)]
pub struct Response<T: Debug + Read>(T);

impl<T: Debug + Read> Read for Response<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<T: Debug + Read> Deref for Response<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HttpSender;

impl HttpSend for HttpSender {
    type Output = reqwest::Response;

    fn execute(&self, client: &Client, request: Request) -> Result<Response<Self::Output>> {
        Ok(Response(client.execute(request)?))
    }
}
