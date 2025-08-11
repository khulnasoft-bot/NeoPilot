use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::platform_impl::web::web_sys::Window;

/// A future that resolves when the next frame is requested.
pub struct RequestAnimationFrameFuture {
    window: Window,
    callback: Closure<dyn FnMut()>,
}

impl RequestAnimationFrameFuture {
    pub fn new(window: Window) -> Self {
        let callback = Closure::wrap(Box::new(|_| {}) as Box<dyn FnMut()>);
        window.request_animation_frame(callback.as_ref().unchecked_ref());
        Self { window, callback }
    }
}

impl Future for RequestAnimationFrameFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.window.request_animation_frame(self.callback.as_ref().unchecked_ref());
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
