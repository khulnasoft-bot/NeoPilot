//! Platform-specific web-sys API bindings for winit.
//!
//! This module contains types and traits that are specific to the web-sys
//! platform. It is not intended to be used directly, and is instead used by
//! the `winit` crate to implement platform-specific functionality.
//!
//! # Examples
//!
//! For examples of how to use this module, see the documentation for the
//! `winit` crate.
//!
//! # Platform-specific notes
//!
//! The web-sys platform is currently experimental and should not be used in
//! production code. It is intended to be used for testing and development
//! purposes only.
//!
//! # Platform-specific types and traits
//!
//! The following types and traits are specific to the web-sys platform:
//!
//! * `WebSysWindow`
//! * `WebSysEventLoop`
//! * `WebSysMonitorHandle`
//! * `WebSysWindowAttributes`
//! * `WebSysWindowId`
//! * `WebSysMonitorId`
//! * `WebSysError`
//!
//! # Platform-specific functions
//!
//! The following functions are specific to the web-sys platform:
//!
//! * `web_sys::window()`
//! * `web_sys::window_id()`
//! * `web_sys::monitor_handle()`
//! * `web_sys::primary_monitor()`
//! * `web_sys::available_monitors()`
//! * `web_sys::monitor_resolution()`
//! * `web_sys::monitor_scale_factor()`
//! * `web_sys::monitor_position()`
//! * `web_sys::monitor_size()`
//! * `web_sys::monitor_is_primary()`
//! * `web_sys::create_window()`
//! * `web_sys::create_event_loop()`
//! * `web_sys::run_event_loop()`
//! * `web_sys::run_return_event_loop()`
//! * `web_sys::poll_events()`
//! * `web_sys::set_window_position()`
//! * `web_sys::set_window_size()`
//! * `web_sys::set_window_title()`
//! * `web_sys::set_window_visible()`
//! * `web_sys::set_window_resizable()`
//! * `web_sys::set_window_decorations()`
//! * `web_sys::set_window_always_on_top()`
//! * `web_sys::set_window_ime_input_mode()`
//! * `web_sys::set_window_ime_position()`
//! * `web_sys::request_redraw()`
//! * `web_sys::redraw_events()`
//! * `web_sys::events_loop_proxy()`
//! * `web_sys::event_loop_proxy()`
//! * `web_sys::window_proxy()`
//! * `web_sys::monitor_proxy()`
//! * `web_sys::drag_window()`
//! * `web_sys::set_cursor_grab()`
//! * `web_sys::set_cursor_visible()`
//! * `web_sys::set_cursor_icon()`
//! * `web_sys::set_cursor_position()`
//! * `web_sys::set_maximized()`
//! * `web_sys::set_fullscreen()`
//! * `web_sys::set_minimized()`
//! * `web_sys::set_outer_position()`
//! * `web_sys::set_outer_size()`
//! * `web_sys::set_position()`
//! * `web_sys::set_size()`
//! * `web_sys::set_title()`
//! * `web_sys::set_resizable()`
//! * `web_sys::set_decorations()`
//! * `web_sys::set_always_on_top()`
//! * `web_sys::set_ime_input_mode()`
//! * `web_sys::set_ime_position()`
//! * `web_sys::request_redraw()`
//! * `web_sys::redraw_events()`
//!
//! # Platform-specific constants
//!
//! The following constants are specific to the web-sys platform:
//!
//! * `web_sys::KEYCODE_BACK`
//! * `web_sys::KEYCODE_FORWARD`
//! * `web_sys::KEYCODE_RELOAD`
//! * `web_sys::KEYCODE_HOME`
//! * `web_sys::KEYCODE_END`
//! * `web_sys::KEYCODE_LEFT`
//! * `web_sys::KEYCODE_UP`
//! * `web_sys::KEYCODE_RIGHT`
//! * `web_sys::KEYCODE_DOWN`
//! * `web_sys::KEYCODE_PAGEUP`
//! * `web_sys::KEYCODE_PAGEDOWN`
//! * `web_sys::KEYCODE_F1`
//! * `web_sys::KEYCODE_F2`
//! * `web_sys::KEYCODE_F3`
//! * `web_sys::KEYCODE_F4`
//! * `web_sys::KEYCODE_F5`
//! * `web_sys::KEYCODE_F6`
//! * `web_sys::KEYCODE_F7`
//! * `web_sys::KEYCODE_F8`
//! * `web_sys::KEYCODE_F9`
//! * `web_sys::KEYCODE_F10`
//! * `web_sys::KEYCODE_F11`
//! * `web_sys::KEYCODE_F12`
//! * `web_sys::KEYCODE_F13`
//! * `web_sys::KEYCODE_F14`
//! * `web_sys::KEYCODE_F15`
//! * `web_sys::KEYCODE_F16`
//! * `web_sys::KEYCODE_F17`
//! * `web_sys::KEYCODE_F18`
//! * `web_sys::KEYCODE_F19`
//! * `web_sys::KEYCODE_F20`
//! * `web_sys::KEYCODE_F21`
//! * `web_sys::KEYCODE_F22`
//! * `web_sys::KEYCODE_F23`
//! * `web_sys::KEYCODE_F24`
//!
//! # Platform-specific errors
//!
//! The following errors are specific to the web-sys platform:
//!
//! * `web_sys::Error`
//! * `web_sys::ErrorKind`
//! * `web_sys::ErrorKind::Generic`
//! * `web_sys::ErrorKind::InvalidArgument`
//! * `web_sys::ErrorKind::InvalidState`
//! * `web_sys::ErrorKind::NoSuchEntry`
//! * `web_sys::ErrorKind::NotSupported`
//! * `web_sys::ErrorKind::NotInitialized`
//! * `web_sys::ErrorKind::Unknown`
//!
//! # Platform-specific traits
//!
//! The following traits are specific to the web-sys platform:
//!
//! * `web_sys::WindowId`
//! * `web_sys::MonitorId`
//! * `web_sys::WindowAttributes`
//! * `web_sys::MonitorHandle`
//! * `web_sys::WindowProxy`
//! * `web_sys::MonitorProxy`
//! * `web_sys::EventsLoopProxy`
//! * `web_sys::EventLoopProxy`
//! * `web_sys::DragWindow`
