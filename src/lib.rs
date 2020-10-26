#![crate_name = "tiny_http"]
#![crate_type = "lib"]
#![forbid(unsafe_code)]

extern crate log;
extern crate ascii;
extern crate chrono;
extern crate chunked_transfer;
extern crate url;

use std::io;
use std::net;
use std::net::{Shutdown, TcpStream, TcpListener};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use client::ClientConnection;
use util::RefinedTcpStream;

pub use common::{HTTPVersion, Header, HeaderField, Method, StatusCode};
pub use request::{ReadWrite, Request};
pub use response::{Response, ResponseBox};

mod client;
mod common;
mod request;
mod response;
mod util;


pub struct Server {
    listener: TcpListener,
    is_shutting_down: Arc<AtomicBool>,
}

impl Server {
    pub fn new(addr: String) -> Result<Server, io::Error>{
        let listener = net::TcpListener::bind(addr)?;

        Ok(Server{
            listener,
            is_shutting_down: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn try_clone(&self) -> Result<Server, io::Error> {
        let listener = self.listener.try_clone()?;

        Ok(Server{
            listener,
            is_shutting_down: self.is_shutting_down.clone(),
        })
    }

    pub fn accept(&self) -> Result<ClientConnection, AcceptError> {
        err_if_false(!self.is_shutting_down.load(Ordering::Relaxed), AcceptError::ShuttingDown())?;

        let (socket, _) = self.listener.accept()
            .map_err(AcceptError::Accept)?;

        let (read_closable, write_closable) = RefinedTcpStream::new(socket);
        Ok(ClientConnection::new(write_closable, read_closable))
    }

    fn shutdown(&mut self) -> Result<(), ShutdownError> {
        self.is_shutting_down.store(true, Ordering::Relaxed);

        let addr = self.listener.local_addr()
            .map_err(ShutdownError::LocalAddr)?;

        // Connect briefly to ourselves to unblock the accept thread
        let stream = TcpStream::connect(addr)
            .map_err(ShutdownError::Connect)?;

        stream.shutdown(Shutdown::Both)
            .map_err(ShutdownError::Shutdown)
    }
}


#[derive(Debug)]
pub enum AcceptError {
    Accept(io::Error),
    ShuttingDown(),
}

#[derive(Debug)]
pub enum ShutdownError {
    LocalAddr(io::Error),
    Connect(io::Error),
    Shutdown(io::Error),
}


impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}


fn err_if_false<E>(value: bool, err: E) -> Result<(), E> {
    if value {
        Ok(())
    } else {
        Err(err)
    }
}
