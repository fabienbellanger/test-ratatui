use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};
use std::rc::Rc;

pub struct AppLayout {
    pub outer_areas: Rc<[Rect]>,
    pub inner_areas: Rc<[Rect]>,
}

impl AppLayout {
    pub fn new(frame: &mut Frame) -> Self {
        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
            .split(frame.area());

        let inner = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(outer[0]);

        AppLayout {
            outer_areas: Rc::from(outer),
            inner_areas: Rc::from(inner),
        }
    }
}
