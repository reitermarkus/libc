pub type c_char = i8;
pub type wchar_t = u32;

pub type c_long = i32;
pub type c_ulong = u32;

s! {
    pub struct sockaddr_un {
        pub sun_family: ::c_short,
        pub sun_path: [::c_char; 108],
    }

    pub struct sockaddr {
        pub sa_len: u8,
        pub sa_family: ::sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct sockaddr_in6 {
        pub sin6_len: u8,
        pub sin6_family: ::sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: ::sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [::c_char; 8],
    }

    pub struct sockaddr_storage {
        pub s2_len: u8,
        pub ss_family: ::sa_family_t,
        pub s2_data1: [::c_char; 2],
        pub s2_data2: [u32; 3],
        pub s2_data3: [u32; 3],
    }
}

pub const AF_UNIX: ::c_short = 1;

pub const POLLIN: ::c_short      = 1 << 0;
pub const POLLRDNORM: ::c_short  = 1 << 1;
pub const POLLRDBAND: ::c_short  = 1 << 2;
pub const POLLPRI: ::c_short     = POLLRDBAND;
pub const POLLOUT: ::c_short     = 1 << 3;
pub const POLLWRNORM: ::c_short  = POLLOUT;
pub const POLLWRBAND: ::c_short  = 1 << 4;
pub const POLLERR: ::c_short     = 1 << 5;
pub const POLLHUP: ::c_short     = 1 << 6;

pub const PTHREAD_STACK_MIN: ::size_t = 200;

extern "C" {
    pub fn writev(
        s: ::c_int,
        iov: *const ::iovec,
        iovcnt: ::c_int,
    ) -> ::c_int;
    pub fn readv(
        fd: ::c_int,
        iov: *const ::iovec,
        iovcnt: ::c_int,
    ) -> ::ssize_t;
}
