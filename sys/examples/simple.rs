use std::{ptr::*, ffi::{CString, c_char}, mem::size_of};
use cef_sys::*;
use winit::{event_loop::EventLoop, window::Window, platform::x11::WindowExtX11};

fn main() {
    unsafe{
        let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
        // convert the strings to raw pointers
        let mut c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
        let arg = cef_main_args_t {
            argc: c_args.len() as i32,
            argv: c_args.as_mut_ptr() as *mut *mut _,
        };

        let mut app = cef_app_t {
            base: _cef_base_ref_counted_t { size: size_of::<_cef_base_ref_counted_t>(), add_ref: None, release: None, has_one_ref: None, has_at_least_one_ref: None },
            on_before_command_line_processing: None,
            on_register_custom_schemes: None,
            get_resource_bundle_handler: None,
            get_browser_process_handler: None,
            get_render_process_handler: None,
        };
        dbg!(cef_execute_process(&arg, &mut app, null_mut()));

        let settings = cef_settings_t {
            size: size_of::<cef_settings_t>(),
            no_sandbox: 1,
            browser_subprocess_path: cef_string_t::default(),
            framework_dir_path: cef_string_t::default(),
            main_bundle_path: cef_string_t::default(),
            chrome_runtime: 0,
            multi_threaded_message_loop: 1,
            external_message_pump: 0,
            windowless_rendering_enabled: 0,
            command_line_args_disabled: 0,
            cache_path: cef_string_t::default(),
            root_cache_path: cef_string_t::default(),
            user_data_path: cef_string_t::default(),
            persist_session_cookies: 0,
            persist_user_preferences: 0,
            user_agent: cef_string_t::default(),
            user_agent_product: cef_string_t::default(),
            locale: cef_string_t::default(),
            log_file: cef_string_t::default(),
            log_severity: cef_log_severity_t::LOGSEVERITY_WARNING,
            javascript_flags: cef_string_t::default(),
            resources_dir_path: cef_string_t::default(),
            locales_dir_path: cef_string_t::default(),
            pack_loading_disabled: 0,
            remote_debugging_port: 800,
            uncaught_exception_stack_size: 0,
            background_color: 0,
            accept_language_list: cef_string_t::default(),
            cookieable_schemes_list: cef_string_t::default(),
            cookieable_schemes_exclude_defaults: 0,
        };
        dbg!(cef_initialize(&arg, &settings, &mut app, null_mut()));

        let el = EventLoop::new();
        let window = Window::new(&el).unwrap();

        let window_info = cef_window_info_t {
            window_name: cef_string_t {
                str_: null_mut(),
                length: 0,
                dtor: None,
            },
            bounds: cef_rect_t {
                x: 0,
                y: 0,
                width: window.inner_size().width as i32,
                height: window.inner_size().height as i32,
            },
            // parent_window: 0,
            parent_window: window.xlib_window().unwrap(),
            windowless_rendering_enabled: 0,
            shared_texture_enabled: 0,
            external_begin_frame_enabled: 0,
            window: 0,
        };

        let url = CString::new("https://www.google.com").unwrap();
        let len = url.as_bytes().len();
        let mut cef_url = cef_string_t {
            str_: null_mut(),
            length: 0,
            dtor: None,
        };
        dbg!(cef_string_utf8_to_utf16(url.into_raw(), len, &mut cef_url));
        dbg!(cef_url);


        let setting = cef_browser_settings_t {
            size: size_of::<cef_browser_settings_t>(),
            windowless_frame_rate: 0,
            standard_font_family: cef_string_t::default(),
            fixed_font_family: cef_string_t::default(),
            serif_font_family: cef_string_t::default(),
            sans_serif_font_family: cef_string_t::default(),
            cursive_font_family: cef_string_t::default(),
            fantasy_font_family: cef_string_t::default(),
            default_font_size: 0,
            default_fixed_font_size: 0,
            minimum_font_size: 0,
            minimum_logical_font_size: 0,
            default_encoding: cef_string_t::default(),
            remote_fonts: cef_state_t::STATE_DEFAULT,
            javascript: cef_state_t::STATE_DEFAULT,
            javascript_close_windows: cef_state_t::STATE_DEFAULT,
            javascript_access_clipboard: cef_state_t::STATE_DEFAULT,
            javascript_dom_paste: cef_state_t::STATE_DEFAULT,
            image_loading: cef_state_t::STATE_DEFAULT,
            image_shrink_standalone_to_fit: cef_state_t::STATE_DEFAULT,
            text_area_resize: cef_state_t::STATE_DEFAULT,
            tab_to_links: cef_state_t::STATE_DEFAULT,
            local_storage: cef_state_t::STATE_DEFAULT,
            databases: cef_state_t::STATE_DEFAULT,
            webgl: cef_state_t::STATE_DEFAULT,
            background_color: 0,
            accept_language_list: cef_string_t::default(),
            chrome_status_bubble: cef_state_t::STATE_DEFAULT,
        };

        let mut client = cef_client_t {
            base: _cef_base_ref_counted_t { size: size_of::<_cef_base_ref_counted_t>(), add_ref: None, release: None, has_one_ref: None, has_at_least_one_ref: None },
            get_audio_handler: None,
            get_command_handler: None,
            get_context_menu_handler: None,
            get_dialog_handler: None,
            get_display_handler: None,
            get_download_handler: None,
            get_drag_handler: None,
            get_find_handler: None,
            get_focus_handler: None,
            get_frame_handler: None,
            get_permission_handler: None,
            get_jsdialog_handler: None,
            get_keyboard_handler: None,
            get_life_span_handler: None,
            get_load_handler: None,
            get_print_handler: None,
            get_render_handler: None,
            get_request_handler: None,
            on_process_message_received: None,
        };

        let _browser = dbg!(cef_browser_host_create_browser(&window_info, &mut client, &cef_url, &setting, null_mut(), null_mut()));
        dbg!(&window_info);
        el.run(|event, _el, control| {
            control.set_wait();
            // cef_do_message_loop_work();
            match event {
                winit::event::Event::NewEvents(winit::event::StartCause::Init) => {
                    // cef_run_message_loop();
                    // cef_shutdown();

                },
                _ => ()
            }

        });
    }
}