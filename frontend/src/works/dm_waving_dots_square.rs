use chrono::Local;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use zoon::{named_color::*, *};

const PI: f64 = std::f64::consts::PI;

// ------ ------
//    States
// ------ ------

#[static_ref]
fn canvas_context() -> &'static Mutable<Option<SendWrapper<CanvasRenderingContext2d>>> {
    Mutable::new(None)
}

#[static_ref]
fn animation_loop() -> &'static Mutable<Option<SendWrapper<AnimationLoop>>> {
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
    start_animation();
}

fn remove_canvas_context() {
    canvas_context().take();
    animation_loop().take();
}

fn start_animation() {
    let alp = AnimationLoop::new(|_| {
        tick();
    });
    animation_loop().set(Some(SendWrapper::new(alp)));
}

#[allow(unused_must_use)]
fn tick() {
    let n_edge = 16;
    let dot_size: f64 = 30.;

    canvas_context().use_ref(|ctx| {
        if let Some(ctx) = ctx.as_ref() {
            let now = Local::now().timestamp_millis();
            ctx.clear_rect(0., 0., 480., 480.);

            for i in 0..(n_edge * n_edge) {
                // (x, y) position of each dot
                let x: f64 = dot_size * ((i % n_edge) as f64) + dot_size / 2.;
                let y: f64 = dot_size * ((i / n_edge) as f64) + dot_size / 2.;

                // radius and color of each dot
                let position_frac = (i % (n_edge + 1)) as f64 / n_edge as f64;
                let time_frac = (now % 1000) as f64 / 999. * dot_size / 2.;
                let radius = position_frac * dot_size / 2.;
                let radius = (time_frac + radius) % (dot_size / 2.);
                let c = 255. - (radius / dot_size * 2. * 255.);
                ctx.set_fill_style(&format!("rgb({},{},{})", c, c, c).apply(JsValue::from));

                ctx.begin_path();
                ctx.arc_with_anticlockwise(x, y, radius, 0., PI * 2., true);
                ctx.fill();
            }
        }
    })
}

// ------ ------
//     View
// ------ ------

pub fn page_content() -> impl Element {
    Canvas::new()
        .width(480)
        .height(480)
        .s(Background::new().color(hsluv!(0., 0., 0.)))
        .s(Borders::all(Border::new().color(GRAY_7)))
        .s(RoundedCorners::all(15))
        .after_insert(set_canvas_context)
        .after_remove(|_| remove_canvas_context())
}
