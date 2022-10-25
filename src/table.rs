use std::time::Duration;
use crossterm::style::Color;

use crate::{levels::Chapter, saves::TimeMap};

#[derive(Clone)]
pub struct TableCell {
    pub text: String,
    pub color: Color,
}

impl TableCell {
    pub fn new_default(text: &str) -> Self {
        Self {
            text: text.to_owned(),
            color: Color::Reset,
        }
    }

    pub fn from_duration(duration: &Duration) -> Self {
        Self::new_default(&format_duration(duration))
    }

    pub fn from_diff(reference: &Duration, other: &Duration, is_best_split: bool) -> Self {
        Self {
            text: format_duration_diff(reference, other),
            color: if is_best_split {
                Color::Blue
            } else if other > reference {
                Color::Red
            } else {
                Color::Green
            }
        }
    }
}

pub struct Table {
    columns: Vec<TableColumn>,
}

impl Table {
    pub fn from_header(columns: Vec<(&str, u16)>) -> Self {
        Self {
            columns: columns.iter().map(|(text, width)| TableColumn { width: *width, cells: vec![TableCell::new_default(*text)]}).collect(),
        }
    }

    pub fn from_default_header() -> Self {
         Table::from_header(vec![("Chapter", 16), ("Time", 7), ("Diff", 5)])
    }

    pub fn from_times(times: &TimeMap, route: &[Chapter]) -> Self {
        let mut table = Self::from_default_header();
        for chapter in route {
            let duration_str = if let Some(time) = times.get(chapter) {
                // TODO: format this properly
                format!("{:?}", time)
            } else {
                "-".to_owned()
            };
            table.push_row(vec![TableCell::new_default(chapter.to_string().as_str()), TableCell::new_default(duration_str.as_str()), TableCell::new_default("-")])
        }
        table
    }

    pub fn push_row(&mut self, cells: Vec<TableCell>) {
        assert!(cells.len() == self.columns.len(), "tried to push row of incorrect size. expected {}, but got {}", self.columns.len(), cells.len());

        for (cell, col) in cells.iter().zip(self.columns.iter_mut()) {
            col.cells.push(cell.clone());
        }
    }

    pub fn columns(&self) -> &Vec<TableColumn> {
        &self.columns
    }
}

pub struct TableColumn {
    pub width: u16,
    cells: Vec<TableCell>,
}

impl TableColumn {
    pub fn cells(&self) -> &Vec<TableCell> {
        &self.cells
    }
}

fn format_duration(duration: &Duration) -> String {
    let secs_total = duration.as_secs();
    let mins = secs_total / 60;
    let secs = secs_total % 60;
    let millis = duration.subsec_millis()/10;
    if duration.as_secs() >= 60 {
        format!("{:02}:{:02}.{:02}", mins, secs, millis)
    } else {
        format!("{:02}.{:02}", secs, millis)
    }
}

fn format_duration_diff(reference: &Duration, other: &Duration) -> String {
    let diff = Duration::from_millis(reference.as_millis().abs_diff(other.as_millis()) as u64);
    let prefix = if other > reference {
        "+"
    } else if other == reference {
        "±"
    } else {
        "-"
    };
    format!("{}{}", prefix, format_duration(&diff))
}
