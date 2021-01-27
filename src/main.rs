use itertools::{EitherOrBoth, Itertools};
use std::error::Error;
use std::io::{stdout, Stdout};
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

    // create random sequence
    let mut rng = rand::thread_rng();
    let mut nums: Vec<u64> = (1..=42).collect();
    nums.shuffle(&mut rng);

    // make a copy for each orst algo
    let mut bubble_nums = nums.clone();
    let mut insert_nums = nums.clone();
    let mut quick_nums = nums.clone();

    // used to track the swap operations
    let mut bubble_swaps = vec![];
    let mut insert_swaps = vec![];
    let mut quick_swaps = vec![];

    // do the orsting
    orst::BubbleOrst.orst(&mut bubble_nums, |i, j| {
        bubble_swaps.push((i, j));
        Exit::No
    });
    orst::InsertionOrst.orst(&mut insert_nums, |i, j| {
        insert_swaps.push((i, j));
        Exit::No
    });
    orst::QuickOrst.orst(&mut quick_nums, |i, j| {
        quick_swaps.push((i, j));
        Exit::No
    });

    let swaps = bubble_swaps
        .into_iter()
        .zip_longest(insert_swaps)
        .zip_longest(quick_swaps);

    let data = nums
        .iter()
        .enumerate()
        .map(|(idx, num)| ((idx + 1).to_string(), *num))
        .collect::<Vec<_>>();

    let mut bubble_data_ref = data
        .iter()
        .map(|(key, num)| (key.as_str(), *num))
        .collect::<Vec<_>>();

    let mut insert_data_ref = bubble_data_ref.clone();
    let mut quick_data_ref = bubble_data_ref.clone();

    for swap in swaps {
        draw(
            &mut terminal,
            &mut bubble_data_ref,
            &mut insert_data_ref,
            &mut quick_data_ref,
        );

        let start = Instant::now();
        let deadline = Duration::from_secs_f64(1.0 / 60.0);

        while start.elapsed() < deadline {
            if event::poll((start + deadline) - Instant::now()).unwrap() {
                if let CEvent::Key(key_event) = event::read().unwrap() {
                    if key_event.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }

        match swap {
            EitherOrBoth::Both(l, (q_from, q_to)) => {
                quick_data_ref.swap(q_from, q_to);
                match l {
                    EitherOrBoth::Both((b_from, b_to), (i_from, i_to)) => {
                        bubble_data_ref.swap(b_from, b_to);
                        insert_data_ref.swap(i_from, i_to);
                    }
                    EitherOrBoth::Left((b_from, b_to)) => bubble_data_ref.swap(b_from, b_to),
                    EitherOrBoth::Right((i_from, i_to)) => insert_data_ref.swap(i_from, i_to),
                }
            }
            EitherOrBoth::Left(l) => match l {
                EitherOrBoth::Both((b_from, b_to), (i_from, i_to)) => {
                    bubble_data_ref.swap(b_from, b_to);
                    insert_data_ref.swap(i_from, i_to);
                }
                EitherOrBoth::Left((b_from, b_to)) => bubble_data_ref.swap(b_from, b_to),
                EitherOrBoth::Right((i_from, i_to)) => insert_data_ref.swap(i_from, i_to),
            },
            EitherOrBoth::Right((q_from, q_to)) => quick_data_ref.swap(q_from, q_to),
        }
    }

    // draw change from last swap call
    draw(
        &mut terminal,
        &mut bubble_data_ref,
        &mut insert_data_ref,
        &mut quick_data_ref,
    );

    // wait until any key is pressed or 42 secs
    let _ = event::poll(Duration::from_secs(42));

    // cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn draw(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    bubble_data_ref: &mut [(&str, u64)],
    insert_data_ref: &mut [(&str, u64)],
    quick_data_ref: &mut [(&str, u64)],
) {
    terminal
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let bubble_chart = BarChart::default()
                .block(Block::default().title("Bubble Orst").borders(Borders::ALL))
                .data(&bubble_data_ref)
                .bar_width(3)
                .bar_style(Style::default().fg(Color::LightYellow))
                .value_style(Style::default().fg(Color::Black).bg(Color::LightYellow));
            f.render_widget(bubble_chart, chunks[0]);

            let insert_chart = BarChart::default()
                .block(
                    Block::default()
                        .title("Insertion Orst")
                        .borders(Borders::ALL),
                )
                .data(&insert_data_ref)
                .bar_width(3)
                .bar_style(Style::default().fg(Color::LightGreen))
                .value_style(Style::default().fg(Color::Black).bg(Color::LightGreen));
            f.render_widget(insert_chart, chunks[1]);

            let quick_chart = BarChart::default()
                .block(Block::default().title("Quick Orst").borders(Borders::ALL))
                .data(&quick_data_ref)
                .bar_width(3)
                .bar_style(Style::default().fg(Color::LightRed))
                .value_style(Style::default().fg(Color::Black).bg(Color::LightRed));
            f.render_widget(quick_chart, chunks[2]);
        })
        .unwrap()
}
