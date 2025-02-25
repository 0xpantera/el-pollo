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
        let res = unsafe { ffi::epoll_create(1) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self { 
            registry: Registry { raw_fd: res },
        })
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
        let fd = self.registry.raw_fd;
        let timeout = timeout.unwrap_or(-1);
        let max_events = events.capacity() as i32;
        let res = unsafe {
            ffi::epoll_wait(fd, events.as_mut_ptr(), max_events, timeout)
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        unsafe { events.set_len(res as usize)}

        Ok(())
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
        let mut event = ffi::Event {
            events: interests as u32,
            epoll_data: token,
        };

        let op = ffi::EPOLL_CTL_ADD;
        let res = unsafe {
            ffi::epoll_ctl(self.raw_fd, op, source.as_raw_fd(), &mut event)
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        let res = unsafe { ffi::close(self.raw_fd) };

        if res < 0 {
            let err = io::Error::last_os_error();
            eprintln!("ERROR: {err:?}");
        }
    }
}