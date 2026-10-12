use std::fmt::{Debug, Display};

pub mod media;
pub mod system;

pub trait Icon: Display + Debug {
    fn to_char(&self) -> String;
    fn optical_offset(&self) -> (f32, f32) {
        (0.0, 0.0) // Default: perfectly centered
    }
}

#[macro_export]
macro_rules! impl_icon {
    ($name:ident, $path:expr, $offset:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name;
        impl $crate::view::icons::Icon for $name {
            fn to_char(&self) -> String {
                nf_icons::nf!($path).to_string()
            }
            fn optical_offset(&self) -> (f32, f32) {
                $offset
            }
        }
        impl Into<String> for $name {
            fn into(self) -> String {
                use $crate::view::icons::Icon;
                self.to_char()
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use $crate::view::icons::Icon;
                write!(f, "{}", self.to_char())
            }
        }
    };
    ($name:ident, $path:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name;
        impl $crate::view::icons::Icon for $name {
            fn to_char(&self) -> String {
                nf_icons::nf!($path).to_string()
            }
        }
        impl Into<String> for $name {
            fn into(self) -> String {
                use $crate::view::icons::Icon;
                self.to_char()
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use $crate::view::icons::Icon;
                write!(f, "{}", self.to_char())
            }
        }
    };
}
