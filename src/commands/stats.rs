use beanstalkd::Beanstalkd;

pub fn all(beanstalkd: &mut Beanstalkd) {
    let stats = beanstalkd.stats().unwrap();
    for (key, value) in stats.iter() {
        println!(" {}: {}", key, value);
    }
}

pub fn get(beanstalkd: &mut Beanstalkd, key: String) {
    let stats = beanstalkd.stats().unwrap();
    let value = stats.get(&key).unwrap();
    print!("{}", value);
}
