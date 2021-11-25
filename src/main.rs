extern crate beanstalkd;
extern crate docopt;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use beanstalkd::Beanstalkd;

mod commands;

static VERSION: &'static str = "0.4.0";
static USAGE: &'static str = "
Beanstalkd CLI

Usage:
    beanstalkd-cli [options] put <message>
    beanstalkd-cli [options] pop
    beanstalkd-cli [options] monitor
    beanstalkd-cli [options] stats [<key>]
    beanstalkd-cli [(--help | --version)]

Commands:
    put <message>      Writes a message to the queue
    pop                Removes and prints the next message in the queue
    monitor            Live monitoring of the queue
    stats [<key>]      Prints all stats or stats for a specific key

Options:
    -h, --host=<host>  Hostname of the beanstalkd server [default: localhost]
    -p, --port=<port>  Port of the beanstalkd server [default: 11300]
    -t, --tube=<tube>  Tube to put/pop from - pop can use multiple tubes comma separated
    --help             Display this message
    -v, --version      Print version info and exit
";

#[derive(Deserialize, Debug)]
struct Args {
    flag_host: String,
    flag_port: u16,
    flag_tube: String,
    cmd_put: bool,
    arg_message: String,
    cmd_pop: bool,
    cmd_monitor: bool,
    cmd_stats: bool,
    arg_key: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(VERSION.to_string())).deserialize())
        .unwrap_or_else(|e| e.exit());

    if !(args.cmd_put || args.cmd_pop || args.cmd_monitor || args.cmd_stats) {
        println!("{}", USAGE.trim());
        return;
    }

    let mut beanstalkd = Beanstalkd::connect(&args.flag_host, args.flag_port)
        .ok()
        .expect("Server not running");

    let mut tubes: Vec<&str> = vec!["default"];

    if !args.flag_tube.is_empty() {
        tubes = args.flag_tube.split(",").collect();
    }

    if args.cmd_put {
        commands::put::put(&mut beanstalkd, args.arg_message, tubes[0]);
    } else if args.cmd_pop {
        commands::pop::pop(&mut beanstalkd, tubes);
    } else if args.cmd_monitor {
        commands::monitor::monitor(&mut beanstalkd);
    } else if args.cmd_stats {
        if args.arg_key.is_some() {
            commands::stats::get(&mut beanstalkd, args.arg_key.unwrap());
        } else {
            commands::stats::all(&mut beanstalkd);
        }
    }
}
