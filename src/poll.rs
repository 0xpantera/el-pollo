use std::{io::{self, Result}, net::TcpStream, os::fd::AsRawFd};
use crate::ffi;

type Events = Vec<ffi::Event>;

/// Represents the event queque
pub struct Poll {
    registry: Registry,
}

impl Poll {
    /// Create new event queue
    pub fn new() -> Result<Self> {
        todo!()
    }

    /// Returns a reference to the registry that we can use
    /// to register interest to be notified about new events
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Blocks the thread it's called on until an
    /// event is ready or it times out, whichever occurs first
    pub fn poll(
        &mut self, 
        events: &mut Events, 
        timeout: Option<i32>
    ) -> Result<()> 
    {
        todo!()
    }
}

/// Handle that allows us to register interest in new events
pub struct  Registry {
    raw_fd: i32,
}

impl Registry {
    /// `source`: only implemented for `TcpStream`
    /// `interests` indicates what kind of events we want our
    /// event queue to keep track of
    pub fn register(
        &self, 
        source: &TcpStream,
        token: usize,
        interests: i32
    ) -> Result<()> 
    {
        todo!()
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        todo!()
    }
}