use beanstalkd::Beanstalkd;

pub fn put(beanstalkd: &mut Beanstalkd, message: String, tube: &str) {
    if tube != "default" {
        let _ = beanstalkd.tube(tube);
    }
    let _ = beanstalkd.put(&message, 0, 0, 10000);
}
