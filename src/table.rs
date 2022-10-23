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

    pub fn from_times(times: TimeMap, route: &[Chapter]) -> Self {
        let mut table = Table::from_header(vec![("Chapter", 16), ("Time", 7), ("Diff", 5)]);
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
