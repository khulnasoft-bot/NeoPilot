use std::sync::atomic::{AtomicBool, Ordering};

use crate::platform_impl::web::web_sys::util::convert_media_query_list;

use super::MediaQueryHandle;

/// A `MediaQueryHandle` implementation for web.
#[derive(Debug)]
pub struct WebSysMediaQueryHandle {
    /// The JavaScript function to call when the media query list changes.
    pub onchange_callback: Option<js_sys::Function>,
    /// The `MediaQueryList` instance.
    pub media_query_list: web_sys::MediaQueryList,
    /// Whether the event listener is currently registered.
    pub onchange_registered: AtomicBool,
}

impl WebSysMediaQueryHandle {
    /// Creates a new `WebSysMediaQueryHandle` instance.
    #[inline]
    pub fn new(media_query_list: web_sys::MediaQueryList) -> Self {
        Self {
            onchange_callback: None,
            media_query_list,
            onchange_registered: AtomicBool::new(false),
        }
    }

    /// Registers the `onchange` event listener if it hasn't been registered yet.
    #[inline]
    pub fn register_onchange_listener(&self) {
        if self.onchange_registered.swap(true, Ordering::SeqCst) {
            return;
        }

        let onchange_callback = self.onchange_callback.as_ref().unwrap().clone();
        let media_query_list = self.media_query_list.clone();

        let onchange_callback = Closure::wrap(Box::new(move || {
            onchange_callback.call1(
                &media_query_list,
                &convert_media_query_list(&media_query_list),
            )
            .expect("onchange callback failed");
        }) as Box<dyn FnMut()>);

        self.media_query_list
            .set_onchange(Some(onchange_callback.as_ref().unchecked_ref()));
    }

    /// Unregisters the `onchange` event listener if it has been registered.
    #[inline]
    pub fn unregister_onchange_listener(&self) {
        if !self.onchange_registered.swap(false, Ordering::SeqCst) {
            return;
        }

        self.media_query_list.set_onchange(None);
    }
}

impl AsRef<web_sys::MediaQueryList> for WebSysMediaQueryHandle {
    #[inline]
    fn as_ref(&self) -> &web_sys::MediaQueryList {
        &self.media_query_list
    }
}

impl MediaQueryHandle for WebSysMediaQueryHandle {
    #[inline]
    fn matches(&self) -> bool {
        self.media_query_list.matches()
    }
}
