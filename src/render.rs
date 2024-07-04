#[allow(non_snake_case)]
use crate::{rc::RcImpl, AccessibilityHandler};
use cef_sys::{
    cef_accessibility_handler_t, _cef_browser_t, _cef_drag_data_t,
    cef_drag_operations_mask_t, cef_horizontal_alignment_t,
    cef_paint_element_type_t, cef_range_t, cef_rect_t, cef_render_handler_t, cef_screen_info_t,
    cef_size_t, cef_string_t, cef_text_input_mode_t,
    cef_touch_handle_state_t,
};
use std::ffi::{c_int, c_void};

/// See [cef_render_handler_t] for more documentation.
pub trait RenderHandler: Sized {
    fn get_accessibility_handler(&self) -> *mut cef_accessibility_handler_t {}
    fn get_root_screen_rect(&self, _browser: *mut _cef_browser_t, _rect: *mut cef_rect_t) -> c_int {}
    fn get_view_rect(&self, _browser: *mut _cef_browser_t, _rect: *mut cef_rect_t) {}
    fn get_screen_point(
        &self,
        browser: *mut _cef_browser_t,
        _viewX: c_int,
        _viewY: c_int,
        _screenX: *mut c_int,
        _screenY: *mut c_int,
    ) -> c_int {
    }
    fn get_screen_info(
        &self,
        browser: *mut _cef_browser_t,
        screen_info: *mut cef_screen_info_t,
    ) -> c_int {
    }
    fn on_popup_show(&self, _browser: *mut _cef_browser_t, _show: c_int) {}
    fn on_popup_size(&self, _browser: *mut _cef_browser_t, _rect: *const cef_rect_t) {}
    fn on_paint(
        &self,
        browser: *mut _cef_browser_t,
        type_: cef_paint_element_type_t,
        dirtyRectsCount: usize,
        dirtyRects: *const cef_rect_t,
        buffer: *const c_void,
 &self,        width: c_int,
        height: c_int,
    ) {
    }
    fn on_accelerated_paint(
        &self,
        browser: *mut _cef_browser_t,
        type_: cef_paint_element_type_t,
        dirtyRectsCount: usize,
        dirtyRects: *const cef_rect_t,
        shared_handle: *mut c_void,
    ) {
    }
    fn get_touch_handle_size(
        &self,
        browser: *mut _cef_browser_t,
        orientation: cef_horizontal_alignment_t,
        size: *mut cef_size_t,
    ) {
    }
    fn on_touch_handle_state_changed(
        &self,
        browser: *mut _cef_browser_t,
        state: *const cef_touch_handle_state_t,
    ) {
    }
    fn start_dragging(
        &self,
        browser: *mut _cef_browser_t,
        drag_data: *mut _cef_drag_data_t,
        allowed_ops: cef_drag_operations_mask_t,
        x: c_int,
        y: c_int,
    ) -> c_int {
    }
    fn update_drag_cursor(
        &self,
        browser: *mut _cef_browser_t,
        operation: cef_drag_operations_mask_t,
    ) {
    }
    fn on_scroll_offset_changed(&self, _browser: *mut _cef_browser_t, _x: f64, _y: f64) {}
    fn on_ime_composition_range_changed(
        &self,
        _browser: *mut _cef_browser_t,
        _selected_range: *const cef_range_t,
        _character_boundsCount: usize,
        _character_bounds: *const cef_rect_t,
    ) {
    }
    fn on_text_selection_changed(
        &self,
        _browser: *mut _cef_browser_t,
        _selected_text: *const cef_string_t,
        _selected_range: *const cef_range_t,
    ) {
    }
    fn on_virtual_keyboard_requested(
        &self,
        _browser: *mut _cef_browser_t,
        _input_mode: cef_text_input_mode_t,
    ) {
    }

    fn into_raw(self) -> *mut cef_render_handler_t {
        let mut object: cef_render_handler_t = unsafe { std::mem::zeroed() };

        // Panal delegate doesn't have any methods. So we skip to view.
        // let view = &mut object.base.base;
        // add_view_delegate_methods!(view);

        // object.on_window_created = Some(on_window_created::<Self>);
        object.get_accessibility_handler = Some(get_accessibility_handler::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

extern "C" fn get_accessibility_handler<I: RenderHandler>(
    this: *mut cef_render_handler_t,
) -> *mut _cef_accessibility_handler_t {
    let obj: &RcImpl<_, I> = RcImpl::get(this);
    // let window = Window(unsafe { RefGuard::from_raw(window) });
    // obj.interface.on_window_created(window);
}

extern "C" fn get_root_screen_rect(
    this: *mut cef_render_handler_t,
    browser: *mut _cef_browser_t,
    rect: *mut cef_rect_t,
) -> c_int {
}

extern "C" fn get_view_rect(
    this: *mut cef_render_handler_t,
    _browser: *mut _cef_browser_t,
    _rect: *mut cef_rect_t,
) {
}
