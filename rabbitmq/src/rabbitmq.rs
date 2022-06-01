use amiquip::{
    AmqpProperties, Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish,
    QueueDeclareOptions, Result, Error,
};


pub struct Rabbitmq {
    rabbitmq_server: String,
}

impl Rabbitmq {
    pub fn init() -> Self {
        Self {
            rabbitmq_server: "amqp://user:pass@localhost:5672".to_string()
        }
    }
    pub fn publish(&self, queue_name: &str, message: String) -> Result<()> {
        let mut connection = Connection::insecure_open(&self.rabbitmq_server)?;
        let channel = connection.open_channel(None)?;

        let _ = channel.queue_declare(
            queue_name,
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
        )?;
        let exchange = Exchange::direct(&channel);

        exchange.publish(Publish::with_properties(
            message.as_bytes(),
            queue_name,
            // delivery_mode 2 makes message persistent
            AmqpProperties::default().with_delivery_mode(2),
        ))?;
        println!("rabbitmq: sent message [{:?}] to {}", message, queue_name);
        connection.close()?;
        Ok(())
    }
    pub fn receive(&self, queue_name: &str) -> Result<Option<String>> {
        let mut connection = Connection::insecure_open(&self.rabbitmq_server)?;
        let channel = connection.open_channel(None)?;

        let queue = channel.queue_declare(
            queue_name,
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
        )?;

        let consumer = queue.consume(ConsumerOptions::default())?;
        if consumer.receiver().is_empty(){
            connection.close()?;
            return Ok(None)
        }
        match consumer.receiver().into_iter().last() {
            Some(data) =>{
                match data {
                    ConsumerMessage::Delivery(delivery) => {
                        let body = String::from_utf8_lossy(&delivery.body);
                        let msg = format!("{}", body);
                        println!("rabbitmq: Received [{:?}]", body);
                        consumer.ack(delivery)?;
                        connection.close()?;
                        return Ok(Some(msg));
                    },
                    _ => {
                        connection.close()?;
                        return Err(Error::ClientClosedChannel)
                    }
                }
            },
            None => {
                println!("rabbitmq: queue empty!");
            }
        }
        connection.close()?;
        Ok(None)
    }
    pub fn publish_to_tx_address(&self, message: String) -> Result<()> {
        let mut connection = Connection::insecure_open(&self.rabbitmq_server)?;
        let channel = connection.open_channel(None)?;

        let _ = channel.queue_declare(
            "tx_address",
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
        )?;
        let exchange = Exchange::direct(&channel);

        exchange.publish(Publish::with_properties(
            message.as_bytes(),
            "tx_address",
            // delivery_mode 2 makes message persistent
            AmqpProperties::default().with_delivery_mode(2),
        ))?;
        println!("rabbitmq: sent message [{:?}] to tx_address", message);
        connection.close()?;
        Ok(())
    }
    pub fn receive_from_tx_address(&self) -> Result<Option<String>> {
        let mut connection = Connection::insecure_open(&self.rabbitmq_server)?;
        let channel = connection.open_channel(None)?;

        let queue = channel.queue_declare(
            "tx_address",
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
        )?;

        let consumer = queue.consume(ConsumerOptions::default())?;
        if consumer.receiver().is_empty(){
            connection.close()?;
            return Ok(None)
        }
        match consumer.receiver().into_iter().last() {
            Some(data) =>{
                match data {
                    ConsumerMessage::Delivery(delivery) => {
                        let body = String::from_utf8_lossy(&delivery.body);
                        let msg = format!("{}", body);
                        println!("rabbitmq: Received [{:?}]", body);
                        consumer.ack(delivery)?;
                        connection.close()?;
                        return Ok(Some(msg));
                    },
                    _ => {
                        connection.close()?;
                        return Err(Error::ClientClosedChannel)
                    }
                }
            },
            None => {
                println!("rabbitmq: queue empty!");
            }
        }
        connection.close()?;
        Ok(None)
    }
}
