use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

struct App {
    cash: i32,
    debt: i32,
    drugs: i32,
    drug_price: i32,
    day: i32,
}

impl App {
    fn new() -> App {
        App {
            cash: 2000,
            debt: 5000,
            drugs: 0,
            drug_price: 0,
            day: 1,
        }
    }

    fn buy_drugs(&mut self, amount: i32) {
        let cost = amount * self.drug_price;
        if cost <= self.cash {
            self.cash -= cost;
            self.drugs += amount;
        }
    }

    fn sell_drugs(&mut self, amount: i32) {
        if amount <= self.drugs {
            let profit = amount * self.drug_price;
            self.cash += profit;
            self.drugs -= amount;
        }
    }

    fn next_day(&mut self) {
        self.day += 1;
        self.drug_price = rand::thread_rng().gen_range(1000..=5000);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Char('b') => {
                    app.buy_drugs(1);
                }
                KeyCode::Char('s') => {
                    app.sell_drugs(1);
                }
                KeyCode::Char('n') => {
                    app.next_day();
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(f.size());

    let info_block = Block::default()
        .title("Drug Wars")
        .borders(Borders::ALL);
    
    let info_text = vec![
        Spans::from(Span::styled(
            format!("Day: {}", app.day),
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Spans::from(Span::raw(format!("Cash: ${}", app.cash))),
        Spans::from(Span::raw(format!("Debt: ${}", app.debt))),
        Spans::from(Span::raw(format!("Drugs: {}", app.drugs))),
        Spans::from(Span::raw(format!("Drug Price: ${}", app.drug_price))),
    ];
    
    let info_paragraph = Paragraph::new(info_text)
        .block(info_block)
        .alignment(tui::layout::Alignment::Left);
    
    f.render_widget(info_paragraph, chunks[0]);

    let actions_block = Block::default()
        .title("Actions")
        .borders(Borders::ALL);

    let actions_text = vec![
        ListItem::new("(B)uy Drugs"),
        ListItem::new("(S)ell Drugs"),
        ListItem::new("(N)ext Day"),
        ListItem::new("(Q)uit"),
    ];

    let actions_list = List::new(actions_text)
        .block(actions_block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_widget(actions_list, chunks[1]);
}