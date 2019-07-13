use rumqtt::{MqttClient, MqttOptions, QoS};
use std::{thread, time::Duration};
use std::time;

fn main() {
    let mqtt_options = MqttOptions::new("test-pubsub1", "192.168.9.15", 51883);
    // let mqtt_options = mqtt_options.set_reconnect_opts(rumqtt::mqttoptions::ReconnectOptions::AfterFirstSuccess(1));
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
      
    mqtt_client.subscribe("#", QoS::AtLeastOnce).unwrap();

    // while let Ok(recv) = notifications.try_recv() {
    while let Ok(recv) = notifications.recv() {
        if notifications.is_empty() {
            break;
        }
        println!("{:?}", recv);
    }
    // for notification in notifications {
    //     match notification {
    //         rumqtt::client::Notification::Publish(pubInfo) => {
    //             println!("{:?}", pubInfo.payload);
    //         },
    //         rumqtt::client::Notification::None => break,
    //         _ => break
    //     };
    //     // println!("{:?}", notification)
    // }
}