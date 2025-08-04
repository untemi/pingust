use crate::error::*;
use colored::{ColoredString, Colorize};
use notify_rust::Urgency;
use std::time::Duration;

pub enum Msg<'a> {
    Recheck(u32, Duration),
    Done,
    Error(&'a Error),
}

impl Msg<'_> {
    pub fn message(&self) -> String {
        match self {
            Msg::Recheck(0, _) => "Connected".to_string(),
            Msg::Recheck(left, _) => format!("Connected, {left} Retry left"),
            Msg::Done => "Conection Stable".to_string(),
            Msg::Error(err) => err.to_string(),
        }
    }

    pub fn duration(&self) -> Option<Duration> {
        match self {
            Msg::Recheck(_, dur) => Some(*dur),
            _ => None,
        }
    }

    pub fn as_urgency(&self) -> Urgency {
        match self {
            Msg::Recheck(0, _) => Urgency::Low,
            Msg::Recheck(_, _) => Urgency::Normal,
            Msg::Done => Urgency::Low,
            Msg::Error(_) => Urgency::Critical,
        }
    }

    pub fn ico(&self) -> ColoredString {
        match self {
            Msg::Recheck(0, _) => "✓".green(),
            Msg::Recheck(_, _) => "◎".blue(),
            Msg::Done => "✓".green(),
            Msg::Error(_) => "●".bright_red(),
        }
    }
}
