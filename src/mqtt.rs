use log::warn;
use parking_lot::Mutex;
use rumqttc::{self, Client, MqttOptions, QoS};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::config::Config;
use crate::Status;

pub struct MqttClient {
    inner: Arc<Mutex<InnerMqttClient>>,
    period_secs: u64,
}

impl MqttClient {
    pub fn new(status: Arc<Mutex<Status>>, config: &Config) -> Self {
        let mqtt_options = MqttOptions::new(
            "smaRSt-fan",
            config.mqtt_broker_address.clone(),
            config.mqtt_broker_port,
        );

        let (client, mut connection) = Client::new(mqtt_options, 10);

        thread::spawn(move || {
            for (_, notification) in connection.iter().enumerate() {
                match notification {
                    Err(_) => {
                        // when broker is disconnected, this causes a lot of CPU load.
                        // Mitigate by not running the event loop all the time in this case.
                        std::thread::sleep(Duration::from_millis(1000));
                    }
                    _ => {
                        // Throttle the event loop a bit.
                        std::thread::sleep(Duration::from_millis(100));
                    }
                }
            }
        });

        let inner = InnerMqttClient {
            client: client,
            status: status,
            topic_rpm: config.mqtt_topic_rpm.clone(),
            topic_speed: config.mqtt_topic_speed.clone(),
            topic_temperature: config.mqtt_topic_temperature.clone(),
        };

        MqttClient {
            inner: Arc::new(Mutex::new(inner)),
            period_secs: config.mqtt_period_secs,
        }
    }

    pub fn start(&self) {
        let inner = self.inner.clone();
        let period_secs = self.period_secs.clone();

        thread::spawn(move || loop {
            {
                let mut locked_inner = inner.lock();
                locked_inner.publish();
            }

            std::thread::sleep(Duration::from_secs(period_secs));
        });
    }
}

struct InnerMqttClient {
    client: Client,
    status: Arc<Mutex<Status>>,
    topic_rpm: String,
    topic_speed: String,
    topic_temperature: String,
}

impl InnerMqttClient {
    fn publish(&mut self) {
        let status: Status;
        {
            let aquired_status = self.status.lock();
            status = aquired_status.clone();
        }

        self.publish_sample(status.rpm, self.topic_rpm.clone());
        self.publish_sample(status.speed, self.topic_speed.clone());
        self.publish_sample(status.temperature, self.topic_temperature.clone())
    }

    fn publish_sample<T: ToString>(&mut self, value: Option<T>, topic: String) {
        if value.is_some() {
            let result =
                self.client
                    .try_publish(topic, QoS::AtMostOnce, false, value.unwrap().to_string());

            match result {
                Err(err) => warn!("Could not publish MQTT message. {}", err),
                _ => {}
            }
        }
    }
}
