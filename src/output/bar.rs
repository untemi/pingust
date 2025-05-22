use crate::msg::Msg;

pub trait Bar {
    fn polybar(&self);
    fn waybar(&self);
}

const POLYBAR_RED: &str = "%{F#c55555}";
const POLYBAR_GREEN: &str = "%{F#a9c474}";
const POLYBAR_BLUE: &str = "%{F#0b79fe}";
const POLYBAR_END: &str = "%{F-}";

const WAYBAR_RED: &str = "<span color='#c55555'>";
const WAYBAR_GREEN: &str = "<span color='#a9c474'>";
const WAYBAR_BLUE: &str = "<span color='#0b79fe'>";
const WAYBAR_END: &str = "</span>";

impl Bar for Msg<'_> {
    fn polybar(&self) {
        let start = match self {
            Msg::Recheck(_, _) => POLYBAR_BLUE,
            Msg::Done => POLYBAR_GREEN,
            Msg::Error(_) => POLYBAR_RED,
        };

        println!("{start}●{POLYBAR_END}")
    }

    fn waybar(&self) {
        let start = match self {
            Msg::Recheck(_, _) => WAYBAR_BLUE,
            Msg::Done => WAYBAR_GREEN,
            Msg::Error(_) => WAYBAR_RED,
        };

        println!("{start}●{WAYBAR_END}")
    }
}
