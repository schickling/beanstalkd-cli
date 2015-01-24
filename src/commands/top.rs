use std::io::stdio::flush;
use std::io::timer::sleep;
use std::time::duration::Duration;

use beanstalkd::Beanstalkd;

pub fn top(beanstalkd: &mut Beanstalkd) {
    let one_sec = Duration::seconds(1);
    let interesting_keys = vec!("current-jobs-ready",
                                "current-workers",
                                "current-producers",
                                "current-connections");
    let mut length = 0;
    loop {
        let stats = beanstalkd.stats().unwrap();
        let mut string = range(0, length).fold(String::new(), |s, _| s + "\x08");
        for key in interesting_keys.iter() {
            string = format!("{}{}: {}, ", string, key, stats.get(&key.to_string()).unwrap());;
        }
        length = string.len() - 2;
        string.truncate(length);
        print!("{}", string);
        flush();
        sleep(one_sec);
    }
}
