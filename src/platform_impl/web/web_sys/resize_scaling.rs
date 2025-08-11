use web_sys::{window, HtmlElement, Window};
use winit::{
    dpi::LogicalSize,
    event::{WindowEvent, WindowScaleFactorChangedCause},
};

pub fn request_resize ScalingFactorChanged(window: &Window, event: &WindowEvent<'_>) {
    let html = window.document().unwrap().document_element().unwrap().dyn_into::<HtmlElement>().unwrap();
    let (width, height) = match event {
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            let scale_factor = window.device_pixel_ratio();
            let logical_size = LogicalSize::new(new_inner_size.width as f64 / scale_factor, new_inner_size.height as f64 / scale_factor);
            (logical_size.width as u32, logical_size.height as u32)
        }
        _ => return,
    };

    let current_width = html.client_width();
    let current_height = html.client_height();

    if current_width != width || current_height != height {
        html.set_client_width(width);
        html.set_client_height(height);
    }
}
