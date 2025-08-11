use std::ops::Deref;

use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry};

/// A handle to an intersection observer.
///
/// This handle can be used to observe the intersection of an element with its
/// root element.
#[derive(Debug)]
pub struct IntersectionHandle {
    observer: IntersectionObserver,
    element: Element,
}

impl Deref for IntersectionHandle {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl IntersectionHandle {
    /// Creates a new intersection handle.
    ///
    /// The created handle will observe the intersection of the given element
    /// with its root element.
    pub fn new(element: Element) -> Self {
        let callback = Closure::wrap(Box::new(move |entries: Vec<IntersectionObserverEntry>| {
            for entry in entries {
                if entry.is_intersecting() {
                    element.request_pointer_lock();
                }
            }
        }) as Box<dyn Fn(Vec<IntersectionObserverEntry>)>);

        let observer = IntersectionObserver::new(callback.as_ref().unchecked_ref(), None);

        callback.forget();

        observer.observe(&element);

        Self { observer, element }
    }
}

impl Drop for IntersectionHandle {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}
