use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use zoon::{named_color::*, *};

// ------ ------
//    States
// ------ ------

#[static_ref]
fn canvas_context() -> &'static Mutable<Option<SendWrapper<CanvasRenderingContext2d>>> {
    Mutable::new(None)
}

// ------ ------
//   Commands
// ------ ------

fn set_canvas_context(canvas: HtmlCanvasElement) {
    let ctx = canvas
        .get_context("2d")
        .unwrap_throw()
        .unwrap_throw()
        .unchecked_into::<CanvasRenderingContext2d>();
    canvas_context().set(Some(SendWrapper::new(ctx)));
}

fn remove_canvas_context() {
    canvas_context().take();
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Borders::all(Border::new().color(GRAY_7)))
        .s(RoundedCorners::all(30))
        .s(Clip::both())
        .item(canvas())
}

fn canvas() -> impl Element {
    Canvas::new()
        .width(300)
        .height(300)
        .after_insert(set_canvas_context)
        .after_remove(|_| remove_canvas_context())
}

// ------ ------
//     Start
// ------ ------

fn main() {
    start_app("app", root);
}
