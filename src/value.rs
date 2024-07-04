use crate::{rc::RefGuard, wrapper};
use cef_sys::cef_value_t;

wrapper!(
    #[doc = "See [cef_value_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct Value(cef_value_t);
    pub fn is_valid(&self);
);
