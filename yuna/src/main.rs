use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    style::{Style, Stylize},
    DefaultTerminal, Frame,
};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tui_big_text::{BigTextBuilder, PixelSize};

fn main() -> color_eyre::Result<()> {
    // Init
    color_eyre::install()?;
    let terminal = ratatui::init();

    // Channel
    let (tx, rx) = mpsc::channel();

    // Event Listener Thread
    let event_sender = tx.clone();
    thread::spawn(move || loop {
        if let Ok(ev) = event::read() {
            event_sender.send(AppEvent::Input(ev)).unwrap();
        }
    });

    // Timer Thread
    let timer_sender = tx.clone();
    thread::spawn(move || {
        let tick_rate = Duration::from_secs(1);
        loop {
            thread::sleep(tick_rate);
            timer_sender.send(AppEvent::Tick).unwrap();
        }
    });

    // Main Thread
    let result = App::new().run(terminal, rx);
    ratatui::restore();
    result
}

pub enum AppEvent {
    Tick,
    Input(Event),
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    counter: i32,
    letter: String,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(
        mut self,
        mut terminal: DefaultTerminal,
        rx: mpsc::Receiver<AppEvent>,
    ) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events(&rx)?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::Full)
            .style(Style::new().blue())
            .lines(vec![
                "Created For Yuna!".red().into(),
                "~~~~~~~~~~~~~~~~~".into(),
                self.letter.clone().green().into(),
            ])
            .centered()
            .build();

        frame.render_widget(big_text, frame.area())
    }

    fn handle_events(&mut self, rx: &mpsc::Receiver<AppEvent>) -> Result<()> {
        match rx.try_recv() {
            Ok(msg) => match msg {
                AppEvent::Tick => {
                    self.counter += 1;
                    Ok(())
                }
                AppEvent::Input(ev) => self.handle_crossterm_events(ev),
            },
            Err(_) => Ok(()),
        }
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self, ev: Event) -> Result<()> {
        match ev {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            (_, KeyCode::Char(c)) => self.letter = String::from(c),
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
