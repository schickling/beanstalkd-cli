use beanstalkd::Beanstalkd;

pub fn all(beanstalkd: &mut Beanstalkd) {
    let stats = beanstalkd.stats().unwrap();
    let mut pairs: Vec<String> = stats.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
    pairs.sort();
    for pair in pairs.iter() {
        println!("{}", pair);
    }
}

pub fn get(beanstalkd: &mut Beanstalkd, key: String) {
    let stats = beanstalkd.stats().unwrap();
    let value = stats.get(&key).unwrap();
    print!("{}", value);
}
