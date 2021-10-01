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
}

impl MqttClient {
    pub fn new(status: Arc<Mutex<Status>>, config: &Config) -> Self {
        let mut mqtt_options =
            MqttOptions::new("smaRSt-fan", config.mqtt_broker.clone(), config.mqtt_port);
        mqtt_options.set_keep_alive(60);

        let (client, mut connection) = Client::new(mqtt_options, 10);

        thread::spawn(move || {
            for (_, _) in connection.iter().enumerate() {
                // TODO: when broker is disconnected, this causes a lot of load.
                // Wait for connecting (or sth.) event, and slow down loop if this occurs
                std::thread::sleep(Duration::from_millis(500));
            }
        });

        let inner = InnerMqttClient {
            client: client,
            status: status,
            topic_rpm: config.mqtt_topic_rpm.clone(),
            topic_speed: config.mqtt_topic_speed.clone(),
            topic_temperature: config.mqtt_topic_temperature.clone()
        };

        MqttClient {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn start(&self) {
        let inner = self.inner.clone();

        thread::spawn(move || loop {
            {
                let mut locked_inner = inner.lock();
                locked_inner.publish();
            }

            std::thread::sleep(Duration::from_secs(10));
        });
    }
}

struct InnerMqttClient {
    client: Client,
    status: Arc<Mutex<Status>>,
    topic_rpm: String,
    topic_speed: String,
    topic_temperature: String
}

impl InnerMqttClient {
    fn publish(&mut self) {
        let status : Status;
        {
            let aquired_status = self.status.lock();
            status = aquired_status.clone();
        }

        self.publish_single_topic(status.rpm, self.topic_rpm.clone());
        self.publish_single_topic(status.speed, self.topic_speed.clone());
        self.publish_single_topic(status.temperature, self.topic_temperature.clone())
    }

    fn publish_single_topic<T: ToString>(&mut self, value: Option<T>, topic: String) {
        if value.is_some() {
            let publish_result = self.client.try_publish(
                topic,
                QoS::AtMostOnce,
                false,
                value.unwrap().to_string(),
            );

            match publish_result {
                Err(err) => warn!("Could not publish MQTT message. {}", err),
                _ => {}
            }
        }
    }
}
