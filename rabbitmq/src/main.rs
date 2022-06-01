pub mod rabbitmq;
use crate::rabbitmq::Rabbitmq;

fn main() {
    let mq = Rabbitmq::init();
    mq.publish("test123", "111".to_string());
    mq.publish("test123", "111".to_string());
    mq.publish("test123", "111".to_string());
    mq.publish("test123", "111".to_string());
    let a = mq.receive("test123");
    println!("{:?}", a);
}
