use beanstalkd::Beanstalkd;

pub fn pop(beanstalkd: &mut Beanstalkd) {
    let (id, message) = beanstalkd.reserve().unwrap();
    println!("{}", message);
    let _ = beanstalkd.delete(id);
}
