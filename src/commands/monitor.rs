use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write};

use beanstalkd::Beanstalkd;

pub fn monitor(beanstalkd: &mut Beanstalkd) {
    let one_sec = Duration::from_secs(1);
    let interesting_keys = vec![
        "current-jobs-ready",
        "current-workers",
        "current-producers",
        "current-connections",
    ];
    let mut length = 0;
    let mut stdout = io::stdout();
    loop {
        let stats = beanstalkd.stats().unwrap();
        let mut string = (0..length).fold(String::new(), |s, _| s + "\x08");
        for key in interesting_keys.iter() {
            string = format!(
                "{}{}: {}, ",
                string,
                key,
                stats.get(&key.to_string()).unwrap()
            );
        }
        length = string.len() - 2;
        string.truncate(length);
        write!(stdout, "{}", string).expect("error writing to stdout");
        let _ = stdout.flush();
        sleep(one_sec);
    }
}
