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
}

fn start_animation() {
    let alp = AnimationLoop::new(|_| {
        tick();
    });
    animation_loop().set(Some(SendWrapper::new(alp)));
}

#[allow(unused_must_use)]
fn tick() {
    canvas_context().use_ref(|ctx| {
        if let Some(ctx) = ctx.as_ref() {
            let now = Local::now();
            let hour = (now.hour() % 12) as f64;
            let minute = now.minute() as f64;
            let second = now.second() as f64;

            ctx.save();
            ctx.clear_rect(0., 0., 300., 300.);
            ctx.translate(150., 150.);
            ctx.scale(0.7, 0.7);
            ctx.rotate(-PI / 2.);
            ctx.set_stroke_style(&"white".apply(JsValue::from));
            ctx.set_fill_style(&"gray".apply(JsValue::from));
            ctx.set_line_cap("round");
            
            // Hour marks
            ctx.save();
            ctx.set_line_width(8.);
            for _ in 0..12 {
                ctx.begin_path();
                ctx.rotate(PI / 6.);
                ctx.move_to(100., 0.);
                ctx.line_to(120., 0.);
                ctx.stroke();
            }
            ctx.restore();

            // Minutes marks
            ctx.save();
            ctx.set_line_width(5.);
            for i in 0..60 {
                if i % 5 != 0 {
                    ctx.begin_path();
                    ctx.move_to(117., 0.);
                    ctx.line_to(120., 0.);
                    ctx.stroke();
                }
                ctx.rotate(PI / 30.);
            }
            ctx.restore();

            // Write Hours
            ctx.save();
            ctx.rotate(
                (PI / 6.) * hour + (PI / 360.) * minute + (PI / 21600.) * second
            );
            ctx.set_line_width(14.);
            ctx.begin_path();
            ctx.move_to(-20., 0.);
            ctx.line_to(80., 0.);
            ctx.stroke();
            ctx.restore();

            // Write Minutes
            ctx.save();
            ctx.rotate((PI / 30.) * minute + (PI / 1800.) * second);
            ctx.set_line_width(10.);
            ctx.begin_path();
            ctx.move_to(-28., 0.);
            ctx.line_to(112., 0.);
            ctx.stroke();
            ctx.restore();

            // Write seconds
            ctx.save();
            ctx.rotate((second * PI) / 30.);
            ctx.set_stroke_style(&"#D40000".apply(JsValue::from));
            ctx.set_fill_style(&"#D40000".apply(JsValue::from));
            ctx.set_line_width(6.);
            ctx.begin_path();
            ctx.move_to(-30., 0.);
            ctx.line_to(83., 0.);
            ctx.stroke();
            ctx.begin_path();
            ctx.arc_with_anticlockwise(0., 0., 10., 0., PI * 2., true);
            ctx.fill();
            ctx.begin_path();
            ctx.arc_with_anticlockwise(95., 0., 10., 0., PI * 2., true);
            ctx.stroke();
            ctx.set_fill_style(&"rgba(0, 0, 0, 0)".apply(JsValue::from));
            ctx.arc_with_anticlockwise(0., 0., 3., 0., PI * 2., true);
            ctx.fill();
            ctx.restore();

            // Clock frame
            ctx.begin_path();
            ctx.set_line_width(14.);
            ctx.set_stroke_style(&"#8cb4ff".apply(JsValue::from));
            ctx.arc_with_anticlockwise(0., 0., 142., 0., PI * 2., true);
            ctx.stroke();

            ctx.restore();
        }
    })
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
