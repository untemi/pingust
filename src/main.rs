use output::term::Term;
use output::{bar::Bar, notif::Notif};

use clap::{Parser, Subcommand};
use error::*;
use msg::Msg;
use std::thread::sleep;
use std::{env, time::Duration};

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
    timeout: u64,

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
        variant: Bars,
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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let adrr = env::var("PINGUST_ADRR").unwrap_or("google.com".to_string());

    let att = args.attempts;
    let mut ok_seq = 0;

    while ok_seq < att || args.infinite {
        // run the ping
        let result = ping::run(adrr.clone(), args.timeout);

        // get the appropriate Msg
        let msg = match &result {
            Ok(dur) => {
                ok_seq += 1;
                match args.infinite {
                    // for -i
                    true => Msg::Recheck(0, *dur),
                    // no -i & if its the last loop
                    false if att == ok_seq => Msg::Done,
                    // no -i & we still counting
                    false => Msg::Recheck(att - ok_seq, *dur),
                }
            }

            Err(e) => {
                ok_seq = 0;

                // for --no-error continue & sleep early
                if args.no_error {
                    sleep(Duration::from_millis(args.gap));
                    continue;
                }

                Msg::Error(e)
            }
        };

        // appropriate based on the command
        match &args.command {
            Commands::Notif => msg.notify()?,

            Commands::Term { minimal } => match minimal {
                true => msg.term_min(),
                false => msg.term_full(),
            },

            Commands::Bar { variant } => match variant {
                Bars::Waybar => msg.waybar(),
                Bars::Polybar => msg.polybar(),
            },
        }

        // dont sleep in the last loop
        if ok_seq < att || args.infinite {
            sleep(Duration::from_millis(args.gap));
        }
    }

    Ok(())
}
