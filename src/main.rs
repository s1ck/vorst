use std::{error::Error};
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
    style::{Color, Modifier, Style},
    Terminal,
    widgets::{BarChart, Block, Borders},
};

enum Exit {
    Yes,
    No,
}

trait Sorter {
    fn orst<T, C>(&self, slice: &mut [T], callback: C)
        where
            T: Ord,
            C: FnMut(usize, usize) -> Exit;
}

pub struct BubbleOrst;

impl Sorter for BubbleOrst {
    fn orst<T, C>(&self, slice: &mut [T], mut callback: C)
        where
            T: Ord,
            C: FnMut(usize, usize) -> Exit,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);

                    if matches!(callback(i, i + 1), Exit::Yes) {
                        return;
                    }

                    swapped = true;
                }
            }
        }
    }
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

    let data = nums.iter().enumerate().map(|(idx, num)| ((idx + 1).to_string(), *num)).collect::<Vec<_>>();
    let mut data_ref = data.iter().map(|(key, num)| (key.as_str(), *num)).collect::<Vec<_>>();

    let callback = |i, j| -> Exit {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
                .split(f.size());
            let barchart = BarChart::default()
                .block(Block::default().title("Data1").borders(Borders::ALL))
                .data(&data_ref)
                .bar_width(3)
                .bar_style(Style::default().fg(Color::Yellow))
                .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
            f.render_widget(barchart, chunks[0]);
        }).unwrap();

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

    // sort
    BubbleOrst.orst(&mut nums, callback);

    // Cleanup
    disable_raw_mode()?;
    execute!( terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture )?;
    terminal.show_cursor()?;

    Ok(())
}


#[test]
fn bubble_orst_works() {
    let mut things = vec![4, 2, 3, 1];
    let mut other_things = things.clone();

    BubbleOrst.orst(&mut things, |i, j| {
        other_things.swap(i, j);
        Exit::No
    });
    assert_eq!(things, vec![1, 2, 3, 4]);
    assert_eq!(things, other_things);
}
