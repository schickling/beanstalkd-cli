use std::io::stdio::flush;
use std::io::timer::sleep;
use std::time::duration::Duration;

use beanstalkd::Beanstalkd;

pub fn top(beanstalkd: &mut Beanstalkd) {
    let one_sec = Duration::seconds(1);
    let mut length = 0;
    loop {
        let mut backspace = String::new();
        for _ in range(0, length) {
            backspace = backspace + "\x08";
        }
        let stats = beanstalkd.stats().unwrap();
        let val = stats.get(&"total-jobs".to_string()).unwrap();
        print!("{}{}", backspace, val);
        flush();
        length = val.len();
        sleep(one_sec);
    }
}
