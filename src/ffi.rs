pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLLIN: i32 = 0x1;
pub const EPOLLET: i32 = 1 << 31;

#[link(name = "C")]
extern "C" {
    ///Syscall to create an epoll queue
    ///`size` muste be >0
    pub fn epoll_create(size: i32) -> i32;
    ///Syscall to close the file descriptor we get when we
    ///create an `epoll` instance
    pub fn close(fd: i32) -> i32;
    ///Control interface to perform operations on our `epoll` instance
    ///This is called to register interest in events on a source.
    ///Supports three main operations: `add`, `modify`, or `delete`
    ///`epfd`: the epoll file descriptor we want to perform ops on
    ///`op`: specifies which operation to perform
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;
    ///Call that blocks current thread and waits until one of two things happen:
    ///we receive a notification that an event has occured or,
    ///it times out.
    ///`epfd`: the epoll file descriptor
    ///`events`: an array of the same `Event` struct used in `epoll_ctl`
    ///This `events` arg gives us information about what event *did* occur
    ///The `data` field contains the same data we passed in when we registered interest
    ///`maxevents`: tells the kernel how many events we have reserved space for in our array.
    /// `timeout`: tells the kernel how long we will wait for events before it will wake
    /// us up again so we don't potentially block forever. 
    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}


/// Used to communicate to the OS in `epoll_ctl` and
/// OS uses same struct to communicate with us in `epoll_wait`
#[derive(Debug)]
#[repr(C, packed)]
pub struct Event {
    // bitmask
    // Just using two flags:
    // `EPOLLIN`: bitflag indicating we're interested in read ops on
    // the file handle.
    // `EPOLLET`: bitflag indicating that we're interested in getting
    // events notified with epoll set to an edge-triggered mode.
    pub(crate) events: u32,
    // Token to identify event
    pub(crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}