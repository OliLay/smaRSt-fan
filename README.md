# smaRSt-fan

A Linux service written for controlling a PWM capable fan with PID control logic and smart connectivity. 

Designed to be device-agnostic by using `sysfs` for accessing PWM and GPIO (for the tacho). Comes with `systemd` integration.

## Features

- Tunable PID control of arbitrary PWM fan
- RPM readout by using tacho wire
- MQTT publishing of RPM, throttle and temperature values
- Systemd service
- .deb package for easy installation

# Build
To build, run 
```
cargo build
```

To build the Debian package, run 
```
cargo deb
```

You may have to install `cargo-deb` at first.

# Installation
1. Follow the build instructions above (no binary available yet)
1. Install the .deb package, e.g. by running `sudo dpkg -i target/debian/smarst-fan_0.1.0_armhf.deb`
1. Configure smaRSt-fan, if needed (see [Configuration](#configuration))
1. Check the status of the service by running `sudo systemctl status smarst-fan`

# Configuration
After installing the service, the default config file will be located in the `/etc/smaRSt-fan` folder. 

You can create your own config file, e.g. called `01-my-config.yaml` and save it in the same folder. Values specified in your config file overwrite the values from the default config file, so you may only specify values that you want to be different from the ones in the default config file.

After changing the configuration, restart the service/application.
