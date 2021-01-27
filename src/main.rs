use std::error::Error;
use std::io::stdout;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::prelude::*;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{BarChart, Block, Borders},
    Terminal,
};

use crate::orst::Orster;

mod orst;

pub enum Exit {
    Yes,
    No,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let mut rng = rand::thread_rng();
    let mut nums: Vec<u64> = (1..=42).collect();
    nums.shuffle(&mut rng);

    let data = nums
        .iter()
        .enumerate()
        .map(|(idx, num)| ((idx + 1).to_string(), *num))
        .collect::<Vec<_>>();

    let mut data_ref = data
        .iter()
        .map(|(key, num)| (key.as_str(), *num))
        .collect::<Vec<_>>();

    let callback = |i, j| -> Exit {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [Constraint::Percentage(100), Constraint::Percentage(100)].as_ref(),
                    )
                    .split(f.size());
                let barchart = BarChart::default()
                    .block(Block::default().title("Bubble").borders(Borders::ALL))
                    .data(&data_ref)
                    .bar_width(3)
                    .bar_style(Style::default().fg(Color::Yellow))
                    .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
                f.render_widget(barchart, chunks[0]);
            })
            .unwrap();

        let start = Instant::now();
        let deadline = Duration::from_secs_f64(1.0 / 60.0);

        while start.elapsed() < deadline {
            if event::poll((start + deadline) - Instant::now()).unwrap() {
                if let CEvent::Key(key_event) = event::read().unwrap() {
                    if key_event.code == KeyCode::Char('q') {
                        return Exit::Yes;
                    }
                }
            }
        }

        // swap refs
        data_ref.swap(i, j);

        Exit::No
    };

    // orst
    orst::QuickOrst.orst(&mut nums, callback);

    // draw change from last swap call
    terminal
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [Constraint::Percentage(100), Constraint::Percentage(100)].as_ref(),
                )
                .split(f.size());
            let barchart = BarChart::default()
                .block(Block::default().title("Bubble").borders(Borders::ALL))
                .data(&data_ref)
                .bar_width(3)
                .bar_style(Style::default().fg(Color::Yellow))
                .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
            f.render_widget(barchart, chunks[0]);
        })
        .unwrap();

    // wait until any key is pressed or 42 secs
    let _ = event::poll(Duration::from_secs(42));

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
