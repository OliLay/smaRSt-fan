use log::warn;
use parking_lot::Mutex;
use rumqttc::{self, Client, MqttOptions, QoS};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::Status;

pub struct MqttClient {
    inner: Arc<Mutex<InnerMqttClient>>,
}

impl MqttClient {
    pub fn new(status: Arc<Mutex<Status>>) -> Self {
        let mut mqtt_options = MqttOptions::new("smaRSt-fan", "localhost", 1883);
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
}

impl InnerMqttClient {
    fn publish(&mut self) {
        let aquired_status = self.status.lock();
        if aquired_status.rpm.is_some() {
            let publish_result = self.client.try_publish(
                "fan/rpm",
                QoS::AtMostOnce,
                false,
                aquired_status.rpm.unwrap().to_string(),
            );

            match publish_result {
                Err(err) => warn!("Could not publish MQTT message. {}", err),
                _ => {}
            }
        }
    }
}
