use std::borrow::Cow;
use std::ops::Deref;

pub type CowStr = Cow<'static, str>;

#[macro_export]
macro_rules! web_sys_wrapper {
    ($x:ident) => {
        #[derive(Debug)]
        pub struct $x(pub web_sys::$x);
        unsafe impl Send for $x {}
        unsafe impl Sync for $x {}

        impl From<web_sys::$x> for $x {
            fn from(value: web_sys::$x) -> Self {
                $x(value)
            }
        }

        impl Deref for $x {
            type Target = web_sys::$x;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

web_sys_wrapper!(Window);

impl Default for Window {
    fn default() -> Self {
        web_sys::window().unwrap().into()
    }
}

web_sys_wrapper!(Document);

impl Default for Document {
    fn default() -> Self {
        web_sys::window().unwrap().document().unwrap().into()
    }
}

web_sys_wrapper!(Event);
