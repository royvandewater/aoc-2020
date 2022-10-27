use std::ops::Range;

mod from_str;

pub struct Rule {
    pub label: String,
    pub ranges: Vec<Range<usize>>,
}
pub type Ticket = Vec<usize>;

pub struct Input {
    pub rules: Vec<Rule>,
    pub your_ticket: Ticket,
    pub nearby_tickets: Vec<Ticket>,
}
