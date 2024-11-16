use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use futures::StreamExt;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::Widget,
    DefaultTerminal, Frame,
};
use tokio::time::{Duration, Instant};
use tui_big_text::{BigText, PixelSize};

#[derive(Debug)]
pub struct App {
    should_quit: bool,
    paused: bool,
    elapsed: Duration,
    current_timer: Instant,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new() -> Self {
        Self {
            should_quit: false,
            paused: false,
            elapsed: Default::default(),
            current_timer: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = Default::default();
        self.current_timer = Instant::now();
    }
    pub fn pause_timer(&mut self) {
        self.elapsed += self.current_timer.elapsed();
        self.paused = true
    }
    pub fn unpause_timer(&mut self) {
        self.current_timer = Instant::now();
        self.paused = false
    }

    pub fn toggle_timer(&mut self) {
        if self.paused {
            self.unpause_timer()
        } else {
            self.pause_timer()
        }
    }

    pub fn get_current_timer(&self) -> Duration {
        (self.current_timer.elapsed() * !self.paused as u32) + self.elapsed
    }

    pub fn display_seconds(&self) -> impl Widget {
        let duration = self.get_current_timer();
        let mut seconds = duration.as_secs();
        let minutes = seconds / 60;
        seconds %= 60;

        let timer = format!("{:02}:{:02}", minutes, seconds);
        BigText::builder()
            .pixel_size(PixelSize::Full)
            .lines(vec![timer.into()])
            .pixel_size(PixelSize::Quadrant)
            .style(Style::new().gray())
            .right_aligned()
            .build()
    }

    pub fn display_millis(&self) -> impl Widget {
        let timer = format!(":{:03}", self.get_current_timer().subsec_millis());
        BigText::builder()
            .pixel_size(PixelSize::Full)
            .lines(vec![timer.into()])
            .pixel_size(PixelSize::Sextant)
            .style(Style::new().gray())
            .left_aligned()
            .build()
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), std::io::Error> {
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| self.draw(frame))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }
        Ok(())
    }

    fn build_layout(&self, body: Rect) -> (Rect, Rect) {
        let body_layout = Layout::horizontal([Constraint::Length(50), Constraint::Fill(1)]);
        let [timer_area, _] = body_layout.areas(body);
        let timer_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
        let [area_seconds, area_millis] = timer_layout.areas(timer_area);
        let millis_layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [_, area_millis] = millis_layout.areas(area_millis);
        (area_seconds, area_millis)
    }

    fn draw(&self, frame: &mut Frame) {
        let (area_seconds, area_millis) = self.build_layout(frame.area());
        frame.render_widget(self.display_seconds(), area_seconds);
        frame.render_widget(self.display_millis(), area_millis);
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                    KeyCode::Char(' ') => self.toggle_timer(),
                    KeyCode::Char('r') => self.reset(),
                    _ => {}
                }
            }
        }
    }
}
