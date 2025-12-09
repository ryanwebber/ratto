use crate::kernel;

pub fn print(args: core::fmt::Arguments) {
    if let Some(console) = kernel().console() {
        _ = write!(console, "{}", args);
    }
}

pub fn console() -> Option<&'static dyn crate::console::Console> {
    kernel().console()
}

#[macro_export]
macro_rules! klog {
    ($string:expr) => ({
        $crate::print::print(format_args_nl!(
            concat!("[DBG] ", $string),
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        $crate::print::print(
            format_args_nl!(
                concat!("[DBG] ", $format_string),
                $($arg)*
            )
        )
    });
}

#[macro_export]
macro_rules! kerr {
    ($string:expr) => ({
        $crate::print::print(format_args_nl!(
            concat!("[ERR] ", $string),
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        $crate::print::print(
            format_args_nl!(
                concat!("[ERR] ", $format_string),
                $($arg)*
            )
        )
    });
}

#[macro_export]
macro_rules! kraw {
    ($string:expr) => ({
        $crate::print::print(format_args_nl!($string));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        $crate::print::print(
            format_args_nl!(
                $format_string,
                $($arg)*
            )
        )
    });
}
