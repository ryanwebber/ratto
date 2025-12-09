use core::fmt::{Debug, Write};

pub trait Console: Debug {
    fn write_str(&self, s: &str) -> core::fmt::Result;
    fn write_fmt(&self, args: core::fmt::Arguments) -> core::fmt::Result {
        let mut adapter = WriteAdapter { console: self };
        adapter.write_fmt(args)
    }
}

struct WriteAdapter<'a, C: Console + ?Sized> {
    console: &'a C,
}

impl<'a, C: Console + ?Sized> core::fmt::Write for WriteAdapter<'a, C> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.console.write_str(s)
    }
}
