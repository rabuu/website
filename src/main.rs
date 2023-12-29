use leptos::*;

use leptos::ev::keydown;
use leptos::logging::log;

use leptos_use::{use_event_listener, use_window};

use web_sys::wasm_bindgen::{JsCast, JsValue};

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    let pong_width = window().inner_width().unwrap().as_f64().unwrap() * 0.9;
    let (left, set_left) = create_signal(pong_width / 4.0);
    let (right, set_right) = create_signal(pong_width / 4.0);
    let (ball_x, set_ball_x) = create_signal(pong_width / 2.0 + 30.);
    let (ball_y, set_ball_y) = create_signal(pong_width / 4.0 + 10.);

    let _ = use_event_listener(use_window(), keydown, move |evt| match evt.key().as_str() {
        "w" => log!("w"),
        "a" => log!("a"),
        "s" => log!("s"),
        "d" => log!("d"),
        _ => (),
    });

    view! {
        <h1>"rbuurman.de"</h1>
        <Pong
          win_width={pong_width as u32}
          paddle_left=left paddle_right=right
          ball_x=ball_x ball_y=ball_y/>
        <p>"Das ist die (unfertige) Website von Rasmus Buurman."</p>
        <hr/>
        <div id="links">
            <a href="https://dev.rbuurman.de">dev</a>
            <a href="https://github.com/rabuu">github</a>
            <a href="https://nc.rbuurman.de">nextcloud</a>
        </div>
    }
}

#[component]
fn Pong(
    win_width: u32,
    paddle_left: ReadSignal<f64>,
    paddle_right: ReadSignal<f64>,
    ball_x: ReadSignal<f64>,
    ball_y: ReadSignal<f64>,
) -> impl IntoView {
    let canvas = html::canvas();

    canvas.set_width(win_width);
    canvas.set_height(win_width / 2);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let width: f64 = canvas.width().into();
    let height: f64 = canvas.height().into();

    context.clear_rect(0.0, 0.0, width, height);
    context.set_fill_style(&JsValue::from_str("white"));

    let line_width = width / 100.;
    context.fill_rect(0.0, 0.0, width, line_width);
    context.fill_rect(0.0, 0.0, line_width, height);
    context.fill_rect(0.0, height - line_width, width, line_width);
    context.fill_rect(width - line_width, 0.0, line_width, height);
    context.fill_rect(
        (width / 2.) - (line_width / 8.),
        0.0,
        line_width / 4.0,
        height,
    );

    let offset = line_width * 3.;
    let paddle_width = line_width * 2.;
    let paddle_height = height / 5.;

    let paddle_left = paddle_left
        .get()
        .clamp(line_width, height - line_width - paddle_height);
    let paddle_right = paddle_right
        .get()
        .clamp(line_width, height - line_width - paddle_height);

    context.fill_rect(offset, paddle_left, paddle_width, paddle_height);
    context.fill_rect(
        width - offset - paddle_width,
        paddle_right,
        paddle_width,
        paddle_height,
    );

    context.fill_rect(ball_x.get(), ball_y.get(), 2. * line_width, 2. * line_width);

    view! { {canvas} }
}
