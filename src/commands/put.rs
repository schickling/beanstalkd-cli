use beanstalkd::Beanstalkd;

pub fn put(beanstalkd: &mut Beanstalkd, message: String) {
    let _ = beanstalkd.put(&message, 0, 0, 10000);
}
