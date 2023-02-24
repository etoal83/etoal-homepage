use chrono::Local;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use zoon::{named_color::*, *};

const PI: f64 = std::f64::consts::PI;
const CANVAS_WIDTH: u32 = 480;
const CANVAS_HEIGHT: u32 = 480;

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
    let n_circle = 8;
    let circle_size = (CANVAS_WIDTH / n_circle) as f64;
    let offset_y = 0. - circle_size / 4.;

    canvas_context().use_ref(|ctx| {
        if let Some(ctx) = ctx.as_ref() {
            let now = Local::now().timestamp_millis();
            ctx.clear_rect(0., 0., CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);

            for i in 0..((4 * n_circle + 1) * (n_circle + 1)) {
                // (x, y) position of each dot
                let x: f64 = if (i / (n_circle + 1)) % 2 == 1 {
                    circle_size * ((i % (n_circle + 1)) as f64) + circle_size / 2.
                } else {
                    circle_size * ((i % (n_circle + 1)) as f64)
                };
                let y: f64 =
                    circle_size * ((i / (n_circle + 1)) as f64) / 4. + circle_size / 2. + offset_y;

                // radius and color of each dot
                let radius = circle_size / 2.;
                let chipping_angle = (now as f64 % 3000.) / 3000. * PI * 2.;
                let offset_angle = (i % (n_circle + 2)) as f64 / (n_circle + 1) as f64 * PI;

                ctx.set_fill_style(&"white".apply(JsValue::from));
                ctx.begin_path();
                ctx.arc_with_anticlockwise(x, y, radius, 0., PI * 2., true);
                ctx.fill();

                ctx.set_line_width(circle_size / 16.);
                ctx.set_stroke_style(&"rgb(59, 93, 160)".apply(JsValue::from));
                for j in 0..5 {
                    let phase_angle = PI / 6. * j as f64;
                    ctx.begin_path();
                    ctx.arc_with_anticlockwise(
                        x,
                        y,
                        radius / 4. * j as f64,
                        chipping_angle + offset_angle + phase_angle,
                        chipping_angle + offset_angle + phase_angle + PI * 2. * 9. / 10.,
                        false,
                    );
                    ctx.stroke();
                }
            }
        }
    })
}

// ------ ------
//     View
// ------ ------

pub fn page_content() -> impl Element {
    Canvas::new()
        .width(CANVAS_WIDTH)
        .height(CANVAS_HEIGHT)
        .s(Background::new().color(hsluv!(0., 0., 0.)))
        .s(Borders::all(Border::new().color(GRAY_7)))
        .after_insert(set_canvas_context)
        .after_remove(|_| remove_canvas_context())
}
