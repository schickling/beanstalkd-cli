extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
extern crate beanstalkd;

use docopt::Docopt;
use beanstalkd::Beanstalkd;

static VERSION: &'static str = "0.0.0";
static USAGE: &'static str = "
Command line Beanstalkd tool

Usage:
    beanstalkd-cli put <message>
    beanstalkd-cli [options]

Options:
    -h, --help       Display this message
    -v, --version    Print version info and exit
";

#[derive(RustcDecodable, Show)]
struct Args {
    cmd_put: bool,
    arg_message: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.version(Some(VERSION.to_string())).decode())
                            .unwrap_or_else(|e| e.exit());

    if (args.cmd_put) {
        let mut beanstalkd = Beanstalkd::localhost().unwrap();
        let message = args.arg_message.as_slice();
        beanstalkd.put(message, 0, 0, 10000);
    } else {
        println!("{}", USAGE.trim());
    }
}
