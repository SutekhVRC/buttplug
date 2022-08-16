pub mod client;

use super::{TestDeviceIdentifier, TestHardwareEvent};
use serde::{Serialize, Deserialize};
use buttplug::{
  core::messages::{ScalarSubcommand, VibrateSubcommand, VectorSubcommand, RotationSubcommand},
  server::device::hardware::HardwareCommand
};

#[derive(Serialize, Deserialize)]
struct TestDevice {
  identifier: TestDeviceIdentifier,
  expected_name: Option<String>,
  expected_display_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
enum TestCommand {
  Messages {
    device_index: u32,
    messages: Vec<TestClientCommand>,
  },
  Commands {
    device_index: u32,
    commands: Vec<HardwareCommand>,
  },
  Events {
    device_index: u32,
    events: Vec<TestHardwareEvent>,
  },
}

#[derive(Serialize, Deserialize, Debug)]
enum TestClientCommand {
  Scalar(Vec<ScalarSubcommand>),
  Vibrate(Vec<VibrateSubcommand>),
  Rotate(Vec<RotationSubcommand>),
  Linear(Vec<VectorSubcommand>),
  Battery {
    expected_power: f64,
    run_async: bool,
  },
  Stop,
  RSSI,
}

#[derive(Serialize, Deserialize)]
pub struct DeviceTestCase {
  devices: Vec<TestDevice>,
  device_config_file: Option<String>,
  user_device_config_file: Option<String>,
  device_init: Option<Vec<TestCommand>>,
  device_commands: Vec<TestCommand>,
}