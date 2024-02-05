use ratatui::{prelude::*, widgets::*};

pub struct CrateInfo {
  ci: crates_io_api::Crate,
}

impl CrateInfo {
  pub fn new(ci: crates_io_api::Crate) -> Self {
    Self { ci }
  }
}

impl Widget for CrateInfo {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let ci = self.ci.clone();

    let mut rows = vec![];

    rows.push(Row::new(vec![Cell::from("Name"), Cell::from(ci.name.clone())]));
    rows.push(Row::new(vec![
      Cell::from("Created At"),
      Cell::from(self.ci.created_at.format("%Y-%m-%d %H:%M:%S").to_string()),
    ]));
    rows.push(Row::new(vec![
      Cell::from("Updated At"),
      Cell::from(self.ci.created_at.format("%Y-%m-%d %H:%M:%S").to_string()),
    ]));
    rows.push(Row::new(vec![Cell::from("Max Version"), Cell::from(self.ci.max_version)]));
    if let Some(description) = self.ci.description {
      rows.push(Row::new(vec![Cell::from("Description"), Cell::from(description)]));
    }
    if let Some(homepage) = self.ci.homepage {
      rows.push(Row::new(vec![Cell::from("Homepage"), Cell::from(homepage)]));
    }
    if let Some(repository) = self.ci.repository {
      rows.push(Row::new(vec![Cell::from("Repository"), Cell::from(repository)]));
    }
    if let Some(recent_downloads) = self.ci.recent_downloads {
      rows.push(Row::new(vec![Cell::from("Recent Downloads"), Cell::from(recent_downloads.to_string())]));
    }
    if let Some(max_stable_version) = self.ci.max_stable_version {
      rows.push(Row::new(vec![Cell::from("Max Stable Version"), Cell::from(max_stable_version)]));
    }

    let widths = [Constraint::Fill(1), Constraint::Fill(4)];
    let table_widget = Table::new(rows, widths).block(Block::default().borders(Borders::ALL));
    Widget::render(table_widget, area, buf);
  }
}
