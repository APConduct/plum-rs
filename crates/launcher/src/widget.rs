use gpui::*;

pub struct WidgetContext<'a> {
    pub window: &'a WindowHandle<()>,
}

pub trait Widget: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn render(&self, cx: &mut &WidgetContext) -> Div;
    fn refresh(&mut self) {}
}
