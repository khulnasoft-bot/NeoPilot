use crate::platform_impl::web::web_sys::Window;
use crate::platform_impl::web::web_sys::WebSysWindowTarget;
use winit::event::{DeviceEvent, Event, StartCause, WindowEvent};

pub(crate) fn handle_resize(window: &Window, event: web_sys::Event) -> Option<Event<WebSysWindowTarget>> {
    let width = window.inner_width();
    let height = window.inner_height();
    Some(Event::WindowEvent {
        window_id: WebSysWindowTarget::dummy().id(),
        event: WindowEvent::Resized(physical_size::SizeU32::new(width, height)),
    })
}

pub(crate) fn handle_device_event(window: &Window, event: web_sys::DeviceMotionEvent) -> Option<Event<WebSysWindowTarget>> {
    let acceleration = event.acceleration();
    let acceleration_including_gravity = event.acceleration_including_gravity();

    let mut device_event = DeviceEvent::Motion {
        accel: match (acceleration, acceleration_including_gravity) {
            (Some(a), Some(b)) => Some([a.x, a.y, a.z]),
            _ => None,
        },
    };

    device_event.set_device_id(0);

    Some(Event::DeviceEvent { device_id: 0, event: device_event })
}

pub(crate) fn handle_start_cause(_window: &Window, _event: web_sys::Event) -> Option<StartCause> {
    None
}
