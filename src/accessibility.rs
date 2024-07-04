#[allow(non_snake_case)]
use crate::{
    rc::{RcImpl, RefGuard},
    Value,
};
use cef_sys::{_cef_accessibility_handler_t, cef_accessibility_handler_t, cef_value_t};

/// See [cef_window_delegate_t] for more documentation.
pub trait AccessibilityHandler: Sized {
    fn on_accessibility_tree_change(&self, _value: Value) {}
    fn on_accessibility_location_change(&self, _value: Value) {}

    fn into_raw(self) -> *mut cef_accessibility_handler_t {
        let mut object: cef_accessibility_handler_t = unsafe { std::mem::zeroed() };

        object.on_accessibility_tree_change = Some(on_accessibility_tree_change::<Self>);
        object.on_accessibility_location_change = Some(on_accessibility_location_change::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

extern "C" fn on_accessibility_tree_change<I: AccessibilityHandler>(
    this: *mut _cef_accessibility_handler_t,
    value: *mut cef_value_t,
) {
    let obj: &RcImpl<_, I> = RcImpl::get(this);
    let value = Value(unsafe { RefGuard::from_raw(value) });
    obj.interface.on_accessibility_tree_change(value);
}

extern "C" fn on_accessibility_location_change<I: AccessibilityHandler>(
    this: *mut _cef_accessibility_handler_t,
    value: *mut cef_value_t,
) {
    let obj: &RcImpl<_, I> = RcImpl::get(this);
    let value = Value(unsafe { RefGuard::from_raw(value) });
    obj.interface.on_accessibility_location_change(value);
}
