use beanstalkd::Beanstalkd;

pub fn put(beanstalkd: &mut Beanstalkd, message: String) {
    let _ = beanstalkd.put(message.as_slice(), 0, 0, 10000);
}
