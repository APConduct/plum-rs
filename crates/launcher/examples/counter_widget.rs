use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};

struct CounterWidget {
    count: usize,
}

impl Render for CounterWidget {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.count;
        div()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .size(px(300.0))
            .bg(rgb(0x222222))
            .text_color(rgb(0xffffff))
            .gap_4()
            .child(format!("Button clicked: {} times", count))
            .child(
                div()
                    .p_4()
                    .bg(rgb(0x4444cc))
                    .rounded_lg()
                    .text_color(rgb(0xffffff))
                    .cursor_pointer()
                    .child("Click me!")
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(|this, _event, _, _| {
                            this.count += 1;
                        }),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(300.), px(200.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| CounterWidget { count: 0 }),
        )
        .unwrap();
    });
}
