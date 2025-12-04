use crate::discovery::{SolutionRegistry, build_registry};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::io;
use std::time::Instant;

pub enum AppState {
    MainMenu,
    YearSelection,
    DaySelection {
        year: u16,
    },
    Running {
        year: u16,
        day: u8,
    },
    Results {
        results: Vec<ResultItem>,
    },
    NewSolution {
        year_input: String,
        day_input: String,
        step: NewSolutionStep,
    },
    BenchmarkInput {
        runs_input: String,
    },
    BenchmarkRunning {
        runs: u32,
        current_day: usize,
        current_run: u32,
        total_days: usize,
        current_times: Vec<std::time::Duration>,
        stats: BenchmarkStats,
    },
    BenchmarkResults {
        stats: BenchmarkStats,
    },
}

#[derive(Clone)]
pub enum NewSolutionStep {
    Year,
    Day,
}

pub struct ResultItem {
    pub year: u16,
    pub day: u8,
    pub part1: String,
    pub part2: String,
    pub time: std::time::Duration,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct DayStats {
    pub year: u16,
    pub day: u8,
    pub times: Vec<std::time::Duration>,
    pub min: std::time::Duration,
    pub max: std::time::Duration,
    pub mean: std::time::Duration,
    pub median: std::time::Duration,
}

#[derive(Clone)]
pub struct YearStats {
    pub year: u16,
    pub days: Vec<DayStats>,
    pub total_time: std::time::Duration,
}

#[derive(Clone)]
pub struct BenchmarkStats {
    pub days: Vec<DayStats>,
    pub years: Vec<YearStats>,
    pub overall_min: std::time::Duration,
    pub overall_max: std::time::Duration,
    pub overall_mean: std::time::Duration,
    pub overall_median: std::time::Duration,
    pub total_time: std::time::Duration,
}

pub struct App {
    pub state: AppState,
    pub registry: SolutionRegistry,
    pub year_list_state: ListState,
    pub day_list_state: ListState,
    pub results_scroll: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let registry = build_registry();
        Self {
            state: AppState::MainMenu,
            registry,
            year_list_state: ListState::default(),
            day_list_state: ListState::default(),
            results_scroll: 0,
            should_quit: false,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

        loop {
            terminal.draw(|f| self.ui(f))?;

            // Check if we're benchmarking - if so, use non-blocking event reading
            let is_benchmarking = matches!(self.state, AppState::BenchmarkRunning { .. });

            if is_benchmarking {
                // Non-blocking event check for benchmarking
                if crossterm::event::poll(std::time::Duration::from_millis(10))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            if self.handle_key(key.code)? {
                                break;
                            }
                        }
                    }
                }
                // Small delay to prevent spinning too fast
                std::thread::sleep(std::time::Duration::from_millis(10));
            } else {
                // Blocking event read for normal operation
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if self.handle_key(key.code)? {
                            break;
                        }
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn handle_key(&mut self, key: KeyCode) -> Result<bool> {
        use std::mem;
        let current_state = mem::replace(&mut self.state, AppState::MainMenu);
        let new_state = match current_state {
            AppState::MainMenu => match key {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
                KeyCode::Char('a') | KeyCode::Enter => {
                    self.run_all();
                    return Ok(false);
                }
                KeyCode::Char('y') => {
                    let years = self.registry.years();
                    if !years.is_empty() {
                        self.year_list_state.select(Some(0));
                    }
                    AppState::YearSelection
                }
                KeyCode::Char('d') => {
                    // Show all days
                    let all_days: Vec<_> = self.registry.all();
                    if !all_days.is_empty() {
                        self.day_list_state.select(Some(0));
                    }
                    AppState::DaySelection { year: 0 }
                }
                KeyCode::Char('n') => AppState::NewSolution {
                    year_input: String::new(),
                    day_input: String::new(),
                    step: NewSolutionStep::Year,
                },
                KeyCode::Char('b') => AppState::BenchmarkInput {
                    runs_input: String::new(),
                },
                _ => AppState::MainMenu,
            },
            AppState::YearSelection => match key {
                KeyCode::Esc => AppState::MainMenu,
                KeyCode::Up => {
                    if let Some(selected) = self.year_list_state.selected() {
                        if selected > 0 {
                            self.year_list_state.select(Some(selected - 1));
                        }
                    }
                    AppState::YearSelection
                }
                KeyCode::Down => {
                    if let Some(selected) = self.year_list_state.selected() {
                        let years = self.registry.years();
                        if selected < years.len().saturating_sub(1) {
                            self.year_list_state.select(Some(selected + 1));
                        }
                    }
                    AppState::YearSelection
                }
                KeyCode::Enter => {
                    if let Some(selected) = self.year_list_state.selected() {
                        let years = self.registry.years();
                        if let Some(&year) = years.get(selected) {
                            let days = self.registry.for_year(year);
                            if !days.is_empty() {
                                self.day_list_state.select(Some(0));
                            }
                            AppState::DaySelection { year }
                        } else {
                            AppState::YearSelection
                        }
                    } else {
                        AppState::YearSelection
                    }
                }
                _ => AppState::YearSelection,
            },
            AppState::DaySelection { year } => match key {
                KeyCode::Esc => {
                    if year == 0 {
                        AppState::MainMenu
                    } else {
                        let years = self.registry.years();
                        if let Some(pos) = years.iter().position(|&y| y == year) {
                            self.year_list_state.select(Some(pos));
                        }
                        AppState::YearSelection
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = self.day_list_state.selected() {
                        if selected > 0 {
                            self.day_list_state.select(Some(selected - 1));
                        }
                    }
                    AppState::DaySelection { year }
                }
                KeyCode::Down => {
                    if let Some(selected) = self.day_list_state.selected() {
                        let days = if year == 0 {
                            self.registry
                                .all()
                                .iter()
                                .map(|(_, d)| *d)
                                .collect::<Vec<_>>()
                        } else {
                            self.registry.for_year(year)
                        };
                        if selected < days.len().saturating_sub(1) {
                            self.day_list_state.select(Some(selected + 1));
                        }
                    }
                    AppState::DaySelection { year }
                }
                KeyCode::Enter => {
                    if let Some(selected) = self.day_list_state.selected() {
                        let (target_year, target_day) = if year == 0 {
                            let all_days = self.registry.all();
                            if let Some(&(y, d)) = all_days.get(selected) {
                                (y, d)
                            } else {
                                return Ok(false);
                            }
                        } else {
                            let days = self.registry.for_year(year);
                            if let Some(&d) = days.get(selected) {
                                (year, d)
                            } else {
                                return Ok(false);
                            }
                        };
                        // Run solution immediately
                        self.run_solution(target_year, target_day);
                        return Ok(false);
                    }
                    AppState::DaySelection { year }
                }
                _ => AppState::DaySelection { year },
            },
            AppState::Running { year, day } => {
                // This state shouldn't be reached in handle_key, but return it anyway
                AppState::Running { year, day }
            }
            AppState::Results { results } => match key {
                KeyCode::Esc | KeyCode::Char('q') => AppState::MainMenu,
                KeyCode::Up => {
                    if self.results_scroll > 0 {
                        self.results_scroll -= 1;
                    }
                    AppState::Results { results }
                }
                KeyCode::Down => {
                    self.results_scroll += 1;
                    AppState::Results { results }
                }
                _ => AppState::Results { results },
            },
            AppState::NewSolution {
                year_input,
                day_input,
                step,
            } => {
                match key {
                    KeyCode::Esc => AppState::MainMenu,
                    KeyCode::Enter => {
                        match step {
                            NewSolutionStep::Year => {
                                if !year_input.is_empty() {
                                    AppState::NewSolution {
                                        year_input: year_input.clone(),
                                        day_input: String::new(),
                                        step: NewSolutionStep::Day,
                                    }
                                } else {
                                    AppState::NewSolution {
                                        year_input,
                                        day_input,
                                        step: NewSolutionStep::Year,
                                    }
                                }
                            }
                            NewSolutionStep::Day => {
                                if !day_input.is_empty() {
                                    // Create the new solution
                                    if let (Ok(year), Ok(day)) =
                                        (year_input.parse::<u16>(), day_input.parse::<u8>())
                                    {
                                        match self.create_new_solution(year, day) {
                                            Ok(_) => {
                                                // Success - show message and return to menu
                                                // For now, just return to menu (user needs to restart to see new solution)
                                                AppState::MainMenu
                                            }
                                            Err(e) => {
                                                // Show error message to user
                                                eprintln!("\n❌ Error: {}\n", e);
                                                eprintln!("Press any key to continue...");
                                                // Return to new solution state so user can try again or cancel
                                                AppState::NewSolution {
                                                    year_input,
                                                    day_input,
                                                    step: NewSolutionStep::Day,
                                                }
                                            }
                                        }
                                    } else {
                                        AppState::NewSolution {
                                            year_input,
                                            day_input,
                                            step: NewSolutionStep::Day,
                                        }
                                    }
                                } else {
                                    AppState::NewSolution {
                                        year_input,
                                        day_input,
                                        step: NewSolutionStep::Day,
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => match step {
                        NewSolutionStep::Year => {
                            let mut new_input = year_input.clone();
                            new_input.push(c);
                            AppState::NewSolution {
                                year_input: new_input,
                                day_input,
                                step: NewSolutionStep::Year,
                            }
                        }
                        NewSolutionStep::Day => {
                            let mut new_input = day_input.clone();
                            new_input.push(c);
                            AppState::NewSolution {
                                year_input,
                                day_input: new_input,
                                step: NewSolutionStep::Day,
                            }
                        }
                    },
                    KeyCode::Backspace => match step {
                        NewSolutionStep::Year => {
                            let mut new_input = year_input.clone();
                            new_input.pop();
                            AppState::NewSolution {
                                year_input: new_input,
                                day_input,
                                step: NewSolutionStep::Year,
                            }
                        }
                        NewSolutionStep::Day => {
                            let mut new_input = day_input.clone();
                            new_input.pop();
                            AppState::NewSolution {
                                year_input,
                                day_input: new_input,
                                step: NewSolutionStep::Day,
                            }
                        }
                    },
                    _ => AppState::NewSolution {
                        year_input,
                        day_input,
                        step: step.clone(),
                    },
                }
            }
            AppState::BenchmarkInput { runs_input } => match key {
                KeyCode::Esc => AppState::MainMenu,
                KeyCode::Enter => {
                    if let Ok(runs) = runs_input.parse::<u32>() {
                        if runs > 0 {
                            self.start_benchmark(runs);
                            return Ok(false);
                        }
                    }
                    AppState::BenchmarkInput { runs_input }
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let mut new_input = runs_input.clone();
                    new_input.push(c);
                    AppState::BenchmarkInput {
                        runs_input: new_input,
                    }
                }
                KeyCode::Backspace => {
                    let mut new_input = runs_input.clone();
                    new_input.pop();
                    AppState::BenchmarkInput {
                        runs_input: new_input,
                    }
                }
                _ => AppState::BenchmarkInput { runs_input },
            },
            AppState::BenchmarkRunning { .. } => {
                // Benchmark is running, continue
                return Ok(false);
            }
            AppState::BenchmarkResults { stats } => match key {
                KeyCode::Esc | KeyCode::Char('q') => AppState::MainMenu,
                KeyCode::Up => {
                    if self.results_scroll > 0 {
                        self.results_scroll -= 1;
                    }
                    AppState::BenchmarkResults { stats }
                }
                KeyCode::Down => {
                    self.results_scroll += 1;
                    AppState::BenchmarkResults { stats }
                }
                _ => AppState::BenchmarkResults { stats },
            },
        };
        self.state = new_state;
        Ok(false)
    }

    fn run_all(&mut self) {
        let all_solutions = self.registry.all();
        let mut results = Vec::new();

        for (year, day) in all_solutions {
            let result = self.run_solution_internal(year, day);
            results.push(result);
        }

        self.state = AppState::Results { results };
    }

    fn run_solution(&mut self, year: u16, day: u8) {
        let result = self.run_solution_internal(year, day);
        self.state = AppState::Results {
            results: vec![result],
        };
    }

    fn run_solution_internal(&self, year: u16, day: u8) -> ResultItem {
        let start = Instant::now();
        let solution = self.registry.get(year, day);

        match solution {
            Some(sol) => {
                let input_path = format!("src/{}/{:02}/input.txt", year, day);
                match std::fs::read_to_string(&input_path) {
                    Ok(input) => {
                        let (part1, part2) = sol.solve(&input);
                        let time = start.elapsed();
                        ResultItem {
                            year,
                            day,
                            part1,
                            part2,
                            time,
                            error: None,
                        }
                    }
                    Err(e) => {
                        let time = start.elapsed();
                        ResultItem {
                            year,
                            day,
                            part1: String::new(),
                            part2: String::new(),
                            time,
                            error: Some(format!("Failed to read input: {}", e)),
                        }
                    }
                }
            }
            None => {
                let time = start.elapsed();
                ResultItem {
                    year,
                    day,
                    part1: String::new(),
                    part2: String::new(),
                    time,
                    error: Some("Solution not found".to_string()),
                }
            }
        }
    }

    fn start_benchmark(&mut self, runs: u32) {
        let all_solutions = self.registry.all();
        let total_days = all_solutions.len();

        let stats = BenchmarkStats {
            days: Vec::new(),
            years: Vec::new(),
            overall_min: std::time::Duration::MAX,
            overall_max: std::time::Duration::ZERO,
            overall_mean: std::time::Duration::ZERO,
            overall_median: std::time::Duration::ZERO,
            total_time: std::time::Duration::ZERO,
        };

        self.state = AppState::BenchmarkRunning {
            runs,
            current_day: 0,
            current_run: 0,
            total_days,
            current_times: Vec::new(),
            stats,
        };
    }

    fn run_benchmark_step(&mut self) {
        if let AppState::BenchmarkRunning {
            ref mut current_day,
            ref mut current_run,
            total_days: _,
            ref mut stats,
            ref mut current_times,
            runs,
        } = self.state
        {
            let all_solutions = self.registry.all();

            if *current_day >= all_solutions.len() {
                // Benchmark complete, calculate statistics
                let stats_clone = stats.clone();
                let final_stats = self.calculate_benchmark_stats(stats_clone, runs);
                self.state = AppState::BenchmarkResults { stats: final_stats };
                return;
            }

            let (year, day) = all_solutions[*current_day];

            // Run one iteration
            if *current_run < runs {
                let start = Instant::now();
                if let Some(sol) = self.registry.get(year, day) {
                    let input_path = format!("src/{}/{:02}/input.txt", year, day);
                    if let Ok(input) = std::fs::read_to_string(&input_path) {
                        let _ = sol.solve(&input);
                        current_times.push(start.elapsed());
                    }
                }
                *current_run += 1;
            } else {
                // All runs for this day complete, save stats and move to next day
                if !current_times.is_empty() {
                    let times = current_times.clone();
                    let day_stats = DayStats {
                        year,
                        day,
                        times: times.clone(),
                        min: *times.iter().min().unwrap(),
                        max: *times.iter().max().unwrap(),
                        mean: calculate_mean(&times),
                        median: calculate_median(&mut times.clone()),
                    };
                    stats.days.push(day_stats);
                }
                *current_day += 1;
                *current_run = 0;
                current_times.clear();
            }
        }
    }

    fn calculate_benchmark_stats(&self, mut stats: BenchmarkStats, _runs: u32) -> BenchmarkStats {
        // Group by year
        use std::collections::HashMap;
        let mut year_map: HashMap<u16, Vec<DayStats>> = HashMap::new();

        for day_stat in stats.days.iter() {
            year_map
                .entry(day_stat.year)
                .or_insert_with(Vec::new)
                .push(day_stat.clone());
        }

        // Calculate year statistics
        let mut years = Vec::new();
        for (year, days) in year_map {
            let total_time: std::time::Duration = days.iter().map(|d| d.mean).sum();
            years.push(YearStats {
                year,
                days,
                total_time,
            });
        }
        years.sort_by_key(|y| y.year);
        stats.years = years;

        // Calculate overall statistics
        let all_times: Vec<std::time::Duration> =
            stats.days.iter().flat_map(|d| d.times.clone()).collect();
        if !all_times.is_empty() {
            stats.overall_min = *all_times.iter().min().unwrap();
            stats.overall_max = *all_times.iter().max().unwrap();
            stats.overall_mean = calculate_mean(&all_times);
            let mut sorted_times = all_times.clone();
            stats.overall_median = calculate_median(&mut sorted_times);
            stats.total_time = all_times.iter().sum();
        }

        stats
    }

    fn ui(&mut self, f: &mut Frame) {
        // Run benchmark step if needed
        if matches!(self.state, AppState::BenchmarkRunning { .. }) {
            self.run_benchmark_step();
        }

        let size = f.area();

        match &self.state {
            AppState::MainMenu => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(3),
                    ])
                    .split(size);

                let title = Paragraph::new("Advent of Code Solutions")
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));

                let menu = Paragraph::new(
                    "[A] Run All\n[Y] Select Year\n[D] Select Day\n[N] New Solution\n[B] Benchmark\n[Q] Quit",
                )
                .block(Block::default().borders(Borders::ALL).title("Menu"))
                .alignment(Alignment::Left);

                let footer = Paragraph::new("Use keyboard shortcuts or arrow keys to navigate")
                    .style(Style::default().fg(Color::DarkGray))
                    .alignment(Alignment::Center);

                f.render_widget(title, chunks[0]);
                f.render_widget(menu, chunks[1]);
                f.render_widget(footer, chunks[2]);
            }
            AppState::YearSelection => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(0)])
                    .split(size);

                let title = Paragraph::new("Select Year")
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));

                let years: Vec<ListItem> = self
                    .registry
                    .years()
                    .iter()
                    .map(|y| ListItem::new(y.to_string()))
                    .collect();

                let list = List::new(years)
                    .block(Block::default().borders(Borders::ALL).title("Years"))
                    .highlight_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    );

                f.render_widget(title, chunks[0]);
                f.render_stateful_widget(list, chunks[1], &mut self.year_list_state);
            }
            AppState::DaySelection { year } => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(0)])
                    .split(size);

                let title_text = if *year == 0 {
                    "Select Day (All Years)".to_string()
                } else {
                    format!("Select Day - Year {}", year)
                };

                let title = Paragraph::new(title_text)
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));

                let days: Vec<ListItem> = if *year == 0 {
                    self.registry
                        .all()
                        .iter()
                        .map(|(y, d)| ListItem::new(format!("Year {} Day {:02}", y, d)))
                        .collect()
                } else {
                    self.registry
                        .for_year(*year)
                        .iter()
                        .map(|d| ListItem::new(format!("Day {:02}", d)))
                        .collect()
                };

                let list = List::new(days)
                    .block(Block::default().borders(Borders::ALL).title("Days"))
                    .highlight_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    );

                f.render_widget(title, chunks[0]);
                f.render_stateful_widget(list, chunks[1], &mut self.day_list_state);
            }
            AppState::Running { year, day } => {
                let text = format!("Running Year {} Day {:02}...", year, day);
                let paragraph = Paragraph::new(text)
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(paragraph, size);
            }
            AppState::Results { results } => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(0)])
                    .split(size);

                let title = Paragraph::new("Results")
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));

                // Calculate maximum widths for alignment
                let max_part1_width = results
                    .iter()
                    .filter_map(|r| {
                        if r.error.is_none() {
                            Some(r.part1.len())
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap_or(0);

                let max_part2_width = results
                    .iter()
                    .filter_map(|r| {
                        if r.error.is_none() {
                            Some(r.part2.len())
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap_or(0);

                let max_time_width = results
                    .iter()
                    .filter_map(|r| {
                        if r.error.is_none() {
                            Some(format!("{:?}", r.time).len())
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap_or(0);

                let result_lines: Vec<String> = results
                    .iter()
                    .skip(self.results_scroll)
                    .map(|r| {
                        if let Some(ref err) = r.error {
                            format!("Year {} Day {:02}: ERROR - {}", r.year, r.day, err)
                        } else {
                            let part1_padded =
                                format!("{:>width$}", r.part1, width = max_part1_width);
                            let part2_padded =
                                format!("{:>width$}", r.part2, width = max_part2_width);
                            let time_str = format!("{:?}", r.time);
                            let time_padded =
                                format!("{:>width$}", time_str, width = max_time_width);
                            format!(
                                "Year {} Day {:02}: Part 1 = {}, Part 2 = {} ({})",
                                r.year, r.day, part1_padded, part2_padded, time_padded
                            )
                        }
                    })
                    .collect();

                let result_text = result_lines.join("\n");
                let results_widget = Paragraph::new(result_text)
                    .block(Block::default().borders(Borders::ALL).title("Results"))
                    .wrap(Wrap { trim: true });

                f.render_widget(title, chunks[0]);
                f.render_widget(results_widget, chunks[1]);
            }
            AppState::NewSolution {
                year_input,
                day_input,
                step,
            } => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Length(5),
                        Constraint::Min(0),
                    ])
                    .split(size);

                let title = Paragraph::new("Create New Solution")
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));

                let prompt = match step {
                    NewSolutionStep::Year => Paragraph::new(format!("Enter Year: {}", year_input))
                        .block(Block::default().borders(Borders::ALL).title("Year"))
                        .style(Style::default().fg(Color::Yellow)),
                    NewSolutionStep::Day => Paragraph::new(format!("Enter Day: {}", day_input))
                        .block(Block::default().borders(Borders::ALL).title("Day"))
                        .style(Style::default().fg(Color::Yellow)),
                };

                let instructions =
                    Paragraph::new("Enter digits, press Enter to continue, Esc to cancel")
                        .style(Style::default().fg(Color::DarkGray))
                        .alignment(Alignment::Center);

                f.render_widget(title, chunks[0]);
                f.render_widget(prompt, chunks[1]);
                f.render_widget(instructions, chunks[2]);
            }
            AppState::BenchmarkInput { runs_input } => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Length(5),
                        Constraint::Min(0),
                    ])
                    .split(size);

                let title = Paragraph::new("Benchmark Solutions")
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));

                let prompt = Paragraph::new(format!("Enter number of runs: {}", runs_input))
                    .block(Block::default().borders(Borders::ALL).title("Runs"))
                    .style(Style::default().fg(Color::Yellow));

                let instructions =
                    Paragraph::new("Enter number, press Enter to start, Esc to cancel")
                        .style(Style::default().fg(Color::DarkGray))
                        .alignment(Alignment::Center);

                f.render_widget(title, chunks[0]);
                f.render_widget(prompt, chunks[1]);
                f.render_widget(instructions, chunks[2]);
            }
            AppState::BenchmarkRunning {
                runs,
                current_day,
                current_run,
                total_days,
                ..
            } => {
                let text = format!(
                    "Running benchmark: {} runs\nDay {}/{} - Run {}/{}",
                    runs,
                    current_day + 1,
                    total_days,
                    current_run + 1,
                    runs
                );
                let paragraph = Paragraph::new(text)
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(paragraph, size);
            }
            AppState::BenchmarkResults { stats } => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(0)])
                    .split(size);

                let title = Paragraph::new("Benchmark Results")
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));

                let mut lines = Vec::new();

                // Overall statistics
                lines.push("=== Overall Statistics ===".to_string());
                lines.push(format!("Min: {:?}", stats.overall_min));
                lines.push(format!("Max: {:?}", stats.overall_max));
                lines.push(format!("Mean: {:?}", stats.overall_mean));
                lines.push(format!("Median: {:?}", stats.overall_median));
                lines.push(format!("Total: {:?}", stats.total_time));
                lines.push("".to_string());

                // Year statistics
                for year_stat in &stats.years {
                    lines.push(format!("=== Year {} ===", year_stat.year));
                    lines.push(format!("Total time: {:?}", year_stat.total_time));
                    lines.push("".to_string());

                    // Day statistics
                    for day_stat in &year_stat.days {
                        lines.push(format!(
                            "Day {:02}: min={:?}, max={:?}, mean={:?}, median={:?}",
                            day_stat.day,
                            day_stat.min,
                            day_stat.max,
                            day_stat.mean,
                            day_stat.median
                        ));
                    }
                    lines.push("".to_string());
                }

                let result_text = lines.join("\n");
                let results_widget = Paragraph::new(result_text)
                    .block(Block::default().borders(Borders::ALL).title("Statistics"))
                    .wrap(Wrap { trim: true });

                f.render_widget(title, chunks[0]);
                f.render_widget(results_widget, chunks[1]);
            }
        }
    }

    fn create_new_solution(&self, year: u16, day: u8) -> Result<()> {
        use std::fs;
        use std::path::Path;

        // Check if solution already exists
        let dir_path = format!("src/{}/{:02}", year, day);
        let dir = Path::new(&dir_path);
        let mod_path = dir.join("mod.rs");

        if mod_path.exists() {
            return Err(anyhow::anyhow!(
                "Solution for Year {} Day {:02} already exists at {}",
                year,
                day,
                mod_path.display()
            ));
        }

        // Check if already registered in discovery
        if self.registry.get(year, day).is_some() {
            return Err(anyhow::anyhow!(
                "Solution for Year {} Day {:02} is already registered",
                year,
                day
            ));
        }

        // Create directory structure
        fs::create_dir_all(dir)?;

        // Create mod.rs with template
        let mod_content = format!(
            r#"use crate::Solution;

pub struct Day{:02};

impl Solution for Day{:02} {{
    fn solve(&self, input: &str) -> (String, String) {{
        // TODO: Implement Part 1
        let part1 = "0".to_string();
        
        // TODO: Implement Part 2
        let part2 = "0".to_string();
        
        (part1, part2)
    }}
    
    fn examples(&self) -> Vec<(String, (String, String))> {{
        vec![
            // TODO: Add example test cases from problem description
            // Example: ("input".to_string(), ("expected_p1".to_string(), "expected_p2".to_string())),
        ]
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;
    use crate::run_examples;
    
    #[test]
    fn test_examples() {{
        let solution = Day{:02};
        run_examples(&solution);
    }}
}}
"#,
            day, day, day
        );
        fs::write(&mod_path, mod_content)?;

        // Create empty input.txt
        let input_path = dir.join("input.txt");
        fs::write(&input_path, "")?;

        // Create year module file if it doesn't exist
        let year_mod_path = format!("src/year{}.rs", year);
        if !Path::new(&year_mod_path).exists() {
            let year_mod_content = format!(
                r#"#[path = "{}/{:02}/mod.rs"]
pub mod day{:02};
"#,
                year, day, day
            );
            fs::write(&year_mod_path, year_mod_content)?;
        } else {
            // Append to existing year module
            let mut year_mod_content = fs::read_to_string(&year_mod_path)?;
            let new_mod_line = format!(
                "\n#[path = \"{}/{:02}/mod.rs\"]\npub mod day{:02};\n",
                year, day, day
            );
            if !year_mod_content.contains(&format!("day{:02}", day)) {
                year_mod_content.push_str(&new_mod_line);
                fs::write(&year_mod_path, year_mod_content)?;
            }
        }

        // Update lib.rs to include year module if needed
        let lib_path = "src/lib.rs";
        let mut lib_content = fs::read_to_string(lib_path)?;
        let year_mod_decl = format!("pub mod year{};", year);
        if !lib_content.contains(&year_mod_decl) {
            // Find where to insert (after other year modules)
            if let Some(pos) = lib_content.rfind("pub mod year") {
                if let Some(newline_pos) = lib_content[pos..].find('\n') {
                    let insert_pos = pos + newline_pos + 1;
                    lib_content.insert_str(insert_pos, &format!("{}\n", year_mod_decl));
                    fs::write(lib_path, lib_content)?;
                }
            }
        }

        // Update discovery.rs to register the new solution
        let discovery_path = "src/discovery.rs";
        let mut discovery_content = fs::read_to_string(discovery_path)?;
        let register_line = format!(
            "    registry.register({}, {}, crate::year{}::day{:02}::Day{:02});",
            year, day, year, day, day
        );
        if !discovery_content.contains(&format!("year{}::day{:02}", year, day)) {
            // Find the build_registry function and add the registration
            if let Some(pos) = discovery_content.find("// Register all solutions") {
                if let Some(newline_pos) = discovery_content[pos..].find('\n') {
                    let insert_pos = pos + newline_pos + 1;
                    discovery_content.insert_str(insert_pos, &format!("    {}\n", register_line));
                    fs::write(discovery_path, discovery_content)?;
                }
            }
        }

        eprintln!("\n✓ Created solution for Year {} Day {:02}\n", year, day);
        eprintln!("Next steps:");
        eprintln!("1. Add your input to: {}", input_path.display());
        eprintln!("2. Implement the solve() method in: {}", mod_path.display());
        eprintln!("3. Add example test cases to the examples() method");
        eprintln!("4. Rebuild the project: cargo build");

        Ok(())
    }
}

fn calculate_mean(times: &[std::time::Duration]) -> std::time::Duration {
    if times.is_empty() {
        return std::time::Duration::ZERO;
    }
    let total: u128 = times.iter().map(|d| d.as_nanos()).sum();
    let mean_nanos = total / times.len() as u128;
    std::time::Duration::from_nanos(mean_nanos as u64)
}

fn calculate_median(times: &mut [std::time::Duration]) -> std::time::Duration {
    if times.is_empty() {
        return std::time::Duration::ZERO;
    }
    times.sort();
    let mid = times.len() / 2;
    if times.len() % 2 == 0 {
        (times[mid - 1] + times[mid]) / 2
    } else {
        times[mid]
    }
}
