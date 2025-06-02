use output::bar::Bar;
use output::notif::Notif;
use output::term::Term;

use clap::{Parser, Subcommand};
use error::*;
use msg::Msg;
use std::{env, time::Duration};
use tokio::time::sleep;

mod error;
mod msg;
mod output;
mod ping;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value_t = 100, global = true)]
    #[arg(help = "ping timeout in ms")]
    timout: u64,

    #[arg(short, long, default_value_t = 6000, global = true)]
    #[arg(help = "gap between tires in ms")]
    gap: u64,

    #[arg(short, long, default_value_t = 3, global = true)]
    #[arg(help = "number of sequential tries")]
    attempts: u32,

    #[arg(short, long, global = true)]
    #[arg(help = "run until the end of the universe")]
    infinite: bool,

    #[arg(long, global = true)]
    #[arg(help = "do nothing if error")]
    no_error: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "notification mode")]
    Notif,

    #[command(about = "system bar module colors")]
    Bar {
        #[command(subcommand)]
        types: Bars,
    },

    #[command(about = "terminal output")]
    Term {
        #[arg(short, long)]
        minimal: bool,
    },
}

#[derive(Subcommand)]
enum Bars {
    Waybar,
    Polybar,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let adrr = env::var("PINGUST_ADRR").unwrap_or("google.com".to_string());

    let att = args.attempts;
    let mut ok_seq = 0;

    while ok_seq < att || args.infinite {
        let result = ping::run(&adrr, args.timout).await;

        let msg = match &result {
            Ok(dur) => {
                ok_seq += 1;
                match () {
                    _ if args.infinite => Msg::Recheck(0, *dur),
                    _ if att == ok_seq => Msg::Done,
                    _ => Msg::Recheck(att - ok_seq, *dur),
                }
            }

            Err(e) => {
                ok_seq = 0;
                if args.no_error {
                    continue;
                }

                Msg::Error(e)
            }
        };

        match args.command {
            Commands::Term { minimal: false } => msg.term_full(),
            Commands::Term { minimal: true } => msg.term_min(),
            Commands::Notif => msg.notify()?,

            Commands::Bar {
                types: Bars::Waybar,
            } => msg.waybar(),

            Commands::Bar {
                types: Bars::Polybar,
            } => msg.polybar(),
        }

        sleep(Duration::from_millis(args.gap)).await;
    }

    Ok(())
}
