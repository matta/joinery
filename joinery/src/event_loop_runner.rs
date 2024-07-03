use crate::{app_driver::AppDriver, Widget};

struct EventLoop {}
struct EventLoopBuilder {}

impl EventLoopBuilder {
    fn build(&mut self) -> Result<EventLoop, EventLoopError> {
        todo!()
    }
}

struct EventLoopError {}

pub fn run(
    // Clearly, this API needs to be refactored, so we don't mind forcing this to be passed in here directly
    // This is passed in mostly to allow configuring the Android app
    mut loop_builder: EventLoopBuilder,
    root_widget: impl Widget,
    app_driver: impl AppDriver + 'static,
) -> Result<(), EventLoopError> {
    let event_loop = loop_builder.build()?;

    run_with(event_loop, root_widget, app_driver)
}

pub fn run_with(
    event_loop: EventLoop,
    root_widget: impl Widget,
    app_driver: impl AppDriver + 'static,
) -> Result<(), EventLoopError> {
    let render_cx = RenderContext::new();
    // TODO: We can't know this scale factor until later?
    let scale_factor = 1.0;
    let mut main_state = MainState {
        render_cx,
        render_root: RenderRoot::new(root_widget, WindowSizePolicy::User, scale_factor),
        renderer: None,
        pointer_state: PointerState::empty(),
        app_driver: Box::new(app_driver),
        proxy: event_loop.create_proxy(),

        window: WindowState::Uninitialized(window),
    };

    // If there is no default tracing subscriber, we set our own. If one has
    // already been set, we get an error which we swallow.
    // By now, we're about to take control of the event loop. The user is unlikely
    // to try to set their own subscriber once the event loop has started.
    let _ = crate::tracing_backend::try_init_tracing();

    event_loop.run_app(&mut main_state)
}
