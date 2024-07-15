use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum WorldEvent {}

impl Display for WorldEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "World event triggered")
    }
}
