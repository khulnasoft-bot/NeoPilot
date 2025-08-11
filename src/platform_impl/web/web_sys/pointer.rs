use std::sync::atomic::{AtomicBool, Ordering};

use web_sys::{Event, HtmlElement, PointerEvent, Window};

use crate::platform_impl::web::web_sys::util::get_pointer_lock_element;

static HAS_POINTER_LOCK: AtomicBool = AtomicBool::new(false);

/// Returns true if the pointer is currently locked to the element.
pub fn is_pointer_locked() -> bool {
    HAS_POINTER_LOCK.load(Ordering::SeqCst)
}

/// Requests the pointer to be locked to the element.
pub fn request_pointer_lock(element: &HtmlElement) -> Result<(), String> {
    if HAS_POINTER_LOCK.load(Ordering::SeqCst) {
        return Err("Pointer is already locked".into());
    }

    let window = element.owner_document().unwrap().default_view().unwrap().dyn_into::<Window>().unwrap();
    let pointer_lock_element = get_pointer_lock_element(element);
    if pointer_lock_element.is_none() {
        return Err("No element to lock the pointer to".into());
    }

    let on_pointer_lock_change = Closure::wrap(Box::new(move |event: &PointerEvent| {
        HAS_POINTER_LOCK.store(event.pointer_id() == 1, Ordering::SeqCst);
    }) as Box<dyn FnMut(&PointerEvent)>);

    window.add_event_listener_with_callback("pointerlockchange", on_pointer_lock_change.as_ref().unchecked_ref())
        .map_err(|_| "Failed to add pointer lock event listener".into())?;

    pointer_lock_element
        .unwrap()
        .request_pointer_lock()
        .map_err(|_| "Failed to request pointer lock".into())?;

    Ok(())
}

/// Releases the pointer lock.
pub fn release_pointer_lock() {
    if !HAS_POINTER_LOCK.load(Ordering::SeqCst) {
        return;
    }

    let window = web_sys::window().unwrap();
    let element = get_pointer_lock_element(window.document().unwrap().body().unwrap()).unwrap();

    element.exit_pointer_lock().unwrap();
    HAS_POINTER_LOCK.store(false, Ordering::SeqCst);
}
