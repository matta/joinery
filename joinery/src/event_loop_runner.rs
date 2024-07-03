use crate::{
    app_driver::AppDriver,
    render_root::{RenderRoot, WindowSizePolicy},
    PointerState, TextEvent, Widget, WindowEvent,
};

use std::io::stdout;

use dpi::PhysicalSize;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        self,
        event::{KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};

type EventLoopError = std::io::Error;

struct MainState {
    render_root: RenderRoot,
    pointer_state: PointerState,
    app_driver: Box<dyn AppDriver>,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    quit: bool,
}
impl MainState {
    fn crossterm_event(&mut self, event: crossterm::event::Event) {
        tracing::info!("event_loop_runner got crossterm event: {:?}", event);
        use crossterm::event::Event;
        match event {
            Event::FocusGained => {
                self.render_root
                    .handle_text_event(TextEvent::FocusChange(true));
            }
            Event::FocusLost => {
                self.render_root
                    .handle_text_event(TextEvent::FocusChange(false));
            }
            Event::Key(event) => {
                // Map to these possible Joinery events:
                //
                // - TextEvent::ModifierChange
                // - TextEvent::KeyboardKey
                //
                if event.kind == KeyEventKind::Press && event.code == KeyCode::Char('q') {
                    tracing::info!("Got q, quitting");
                    self.quit = true;
                }
                self.crossterm_key_event(event);
            }
            Event::Mouse(event) => {
                tracing::warn!("Ignoring mouse event: {:?}", event);
            }
            Event::Paste(event) => {
                tracing::warn!("Ignoring paste event: {:?}", event);
            }
            Event::Resize(width, height) => {
                let width: u32 = width.into();
                let height: u32 = height.into();
                let size = PhysicalSize::new(width, height);
                self.render_root
                    .handle_window_event(WindowEvent::Resize(size));
            }
        }
    }

    fn crossterm_key_event(&self, event: crossterm::event::KeyEvent) {
        // Unpack the event
        let crossterm::event::KeyEvent {
            code,
            modifiers,
            kind,
            state,
        } = event;

        // TODO: check if Joinery can use the information in `state`
        let _ = state;
    }
}

pub fn run(
    // Clearly, this API needs to be refactored, so we don't mind forcing this to be passed in here directly
    // This is passed in mostly to allow configuring the Android app
    root_widget: impl Widget,
    app_driver: impl AppDriver + 'static,
) -> Result<(), EventLoopError> {
    run_with(root_widget, app_driver)
}

pub fn run_with(
    root_widget: impl Widget,
    app_driver: impl AppDriver + 'static,
) -> Result<(), EventLoopError> {
    // If there is no default tracing subscriber, we set our own. If one has
    // already been set, we get an error which we swallow.
    // By now, we're about to take control of the event loop. The user is unlikely
    // to try to set their own subscriber once the event loop has started.
    let _ = crate::tracing_backend::try_init_tracing();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // TODO: We can't know this scale factor until later?
    let scale_factor = 1.0;
    let render_root = RenderRoot::new(root_widget, WindowSizePolicy::User, scale_factor);

    let mut main_state = MainState {
        render_root,
        pointer_state: PointerState::empty(),
        app_driver: Box::new(app_driver),
        terminal,
        quit: false,
    };

    run_app(&mut main_state)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    #[cfg(any())]
    {
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

    Ok(())
}

fn run_app(main_state: &mut MainState) -> Result<(), std::io::Error> {
    while !main_state.quit {
        main_state.terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;
        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            main_state.crossterm_event(crossterm::event::read()?);
        }
    }
    Ok(())
}
