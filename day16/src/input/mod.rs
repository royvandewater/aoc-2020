use std::ops::Range;

mod from_str;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    pub label: String,
    pub ranges: Vec<Range<usize>>,
    pub position: Option<usize>,
}
pub type Ticket = Vec<usize>;

#[derive(Clone)]
pub struct Input {
    pub rules: Vec<Rule>,
    pub your_ticket: Ticket,
    pub nearby_tickets: Vec<Ticket>,
}
