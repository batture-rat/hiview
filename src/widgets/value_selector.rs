use std::fmt::Display;

use crate::app::state::{FocusedPane, State};
use notatin::cell_key_value::CellKeyValue;
use notatin::cell_value::CellValue;
use ratatui::prelude::Alignment;
use ratatui::style::Color;
use ratatui::text::Text;
use ratatui::widgets::StatefulWidget;
use ratatui::{
    style::*,
    symbols::border,
    text::Line,
    widgets::{block::*, *},
};

pub struct ValueCellPreview(CellValue);

impl Display for ValueCellPreview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            CellValue::None => f.write_str("<NO VALUE>"),
            CellValue::U32(val) => f.write_str(format!("REG_DWORD: {}", val).as_str()),
            CellValue::U64(val) => f.write_str(format!("REG_QWORD: {}", val).as_str()),
            CellValue::I32(val) => f.write_str(format!("REG_DWORD: {}", val).as_str()),
            CellValue::I64(val) => f.write_str(format!("REG_QWORD: {}", val).as_str()),
            CellValue::String(s) => f.write_str(format!("REG_SZ: \"{}\"", s.clone()).as_str()),
            CellValue::Error => f.write_str("ERROR DECODING VALUE"),
            CellValue::Binary(_) => f.write_str("BINARY DATA"),
            CellValue::MultiString(strs) => f.write_str(
                format!(
                    "REG_MULTI_SZ: {}",
                    strs.iter()
                        .map(|s| format!("\"{}\"", s))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
                .as_str(),
            ),
        }
    }
}

fn make_table_block(focused: bool) -> Block<'static> {
    let title = Title::from("values".to_string());
    let instructions = Title::from(Line::from(vec![
        " Next Value ".into(),
        "<j>".blue().bold(),
        " Previous Value ".into(),
        "<k>".blue().bold(),
    ]));
    Block::default()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(match focused {
            true => border::THICK,
            false => border::PLAIN,
        })
        .border_style(match focused {
            true => Color::Green,
            false => Color::default(),
        })
}

struct UIValueSet<'a> {
    values: &'a Vec<CellKeyValue>,
    focused: bool,
}

impl From<UIValueSet<'_>> for Table<'_> {
    fn from(value: UIValueSet) -> Self {
        let rows: Vec<Row> = value
            .values
            .iter()
            .map(|value| {
                Row::new(vec![
                    Cell::new(value.get_pretty_name()),
                    Cell::new(ValueCellPreview(value.get_content().0).to_string()),
                ])
            })
            .collect::<Vec<Row>>();

        Table::new(rows, vec![80, 80])
            .block(make_table_block(value.focused))
            .highlight_style(Style::new().add_modifier(Modifier::BOLD))
            .highlight_symbol(Text::from("|").blue())
    }
}

pub struct ValueSelector {}

impl StatefulWidget for &mut ValueSelector {
    type State = State;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        <Table as StatefulWidget>::render(
            UIValueSet {
                values: &state.navigation.current_values,
                focused: state.focused_pane == FocusedPane::ValueSelector,
            }
            .into(),
            area,
            buf,
            &mut state.navigation.table_states.value_selector_state,
        );
    }
}
