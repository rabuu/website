use leptos::*;

use leptos::ev::keydown;
use leptos::logging::log;

use leptos_use::{use_event_listener, use_interval_fn, use_window, utils::Pausable};

use web_sys::wasm_bindgen::{JsCast, JsValue};

mod pong;
use pong::Pong;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    let width = window().inner_width().unwrap().as_f64().unwrap() * 0.9;
    let (pong, set_pong) = create_signal(Pong::new(width, 6.0, 3.0));

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn(
        move || {
            set_pong.update(Pong::tick);
            // log!("{:?}", pong.get());
        },
        60,
    );

    let _ = use_event_listener(use_window(), keydown, move |evt| match evt.key().as_str() {
        "k" | "w" | "ArrowUp" if is_active.get() => set_pong.update(|p| p.scroll(-5.0)),
        "j" | "s" | "ArrowDown" if is_active.get() => set_pong.update(|p| p.scroll(5.0)),
        "p" if is_active.get() => pause(),
        "r" if !is_active.get() => resume(),
        key => log!("Pressed key: '{}'", key),
    });

    view! {
        <h1>"rbuurman.de"</h1>
        <p>""</p>
        <PongDisplay pong={pong}/>
        <p>"Das ist die Website von Rasmus Buurman."</p>
        <hr/>
        <div class="links">
            <a href="https://dev.rbuurman.de">dev</a>
            <a href="https://github.com/rabuu">github</a>
            <a href="https://nc.rbuurman.de">nextcloud</a>
        </div>
    }
}

#[component]
fn PongDisplay(pong: ReadSignal<Pong>) -> impl IntoView {
    let (canvas, set_canvas) = create_signal(html::canvas());

    create_effect(move |_| {
        let pong = pong.get();

        set_canvas.update(|c| {
            c.set_width(pong.width as u32);
            c.set_height(pong.height as u32);
        });

        let ctx = canvas
            .get()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        ctx.clear_rect(0.0, 0.0, pong.width, pong.height);
        ctx.set_fill_style(&JsValue::from_str("white"));

        ctx.fill_rect(0.0, 0.0, pong.width, pong.line_width);
        ctx.fill_rect(0.0, 0.0, pong.line_width, pong.height);
        ctx.fill_rect(
            0.0,
            pong.height - pong.line_width,
            pong.width,
            pong.line_width,
        );
        ctx.fill_rect(
            pong.width - pong.line_width,
            0.0,
            pong.line_width,
            pong.height,
        );
        ctx.fill_rect(
            (pong.width / 2.) - (pong.line_width / 8.),
            0.0,
            pong.line_width / 4.0,
            pong.height,
        );

        ctx.fill_rect(
            pong.paddle_left.x,
            pong.paddle_left.y,
            pong.paddle_width,
            pong.paddle_height,
        );
        ctx.fill_rect(
            pong.paddle_right.x,
            pong.paddle_right.y,
            pong.paddle_width,
            pong.paddle_height,
        );

        ctx.fill_rect(pong.ball.x, pong.ball.y, pong.ball_size, pong.ball_size);
    });

    view! { <div>{canvas}</div> }
}
