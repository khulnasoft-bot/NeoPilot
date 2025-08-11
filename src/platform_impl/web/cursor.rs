use std::convert::TryInto;
use web_sys::{window, HtmlCanvasElement};
use winit::{
    dpi::PhysicalPosition,
    event::{CursorGrabMode, DeviceId, ElementState, Event, MouseButton, WindowEvent},
};

pub fn process_events(
    canvas: &HtmlCanvasElement,
    events: Vec<WindowEvent>,
    _window_id: DeviceId,
) -> Vec<Event> {
    let mut events = events.into_iter();
    let mut result = Vec::new();

    while let Some(event) = events.next() {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let physical = PhysicalPosition::new(
                    position.x.try_into().unwrap(),
                    position.y.try_into().unwrap(),
                );
                result.push(Event::WindowEvent {
                    window_id: event.window_id,
                    event: WindowEvent::CursorMoved { position: physical },
                });
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let ElementState::Pressed = state {
                    let rect = canvas.get_bounding_client_rect();
                    let x = rect.left() + (canvas.client_width() as f64 * 0.5);
                    let y = rect.top() + (canvas.client_height() as f64 * 0.5);
                    window().unwrap().focus().unwrap();
                    canvas.request_pointer_lock().unwrap();
                    result.push(Event::DeviceEvent {
                        device_id: event.window_id,
                        event: DeviceEvent::Motion {
                            position: PhysicalPosition::new(x as f64, y as f64),
                        },
                    });
                } else {
                    canvas.exit_pointer_lock().unwrap();
                    result.push(Event::DeviceEvent {
                        device_id: event.window_id,
                        event: DeviceEvent::Button {
                            button: match button {
                                MouseButton::Left => winit::event::MouseButton::Left,
                                MouseButton::Right => winit::event::MouseButton::Right,
                                MouseButton::Middle => winit::event::MouseButton::Middle,
                                MouseButton::Other(n) => winit::event::MouseButton::Other(n),
                            },
                            state,
                        },
                    });
                }
            }
            WindowEvent::CursorGrabModeChanged { mode, .. } => {
                if let CursorGrabMode::Confined = mode {
                    canvas.request_pointer_lock().unwrap();
                } else {
                    canvas.exit_pointer_lock().unwrap();
                }
            }
            _ => result.push(Event::WindowEvent { window_id: event.window_id, event }),
        }
    }

    result
