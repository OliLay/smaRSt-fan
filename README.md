# smaRSt-fan

A Linux service written for controlling a PWM capable fan with PID control logic and smart connectivity. 

Designed to be device-agnostic by using `sysfs` for accessing PWM and GPIO (for the tacho). Comes with `systemd` integration.

# Build
To build, run 
```
cargo build
```

To build the Debian package that installs the systemd service, run 
```
cargo deb
```

You may have to install `cargo-deb` at first.

# Configure
After installing the service, the default config file will be located in the `/etc/smaRSt-fan` folder. 

You can create your own config file, e.g. called `01-my-config.yaml` and save it in the same folder. Values specified in your config file overwrite the values from the default config file, so you may only specify values that you want to be different from the ones in the default config file.

## ToDo

- [x] Basics: Control fan with PWM, read out actual RPM
- [x] PID Control
- [x] Service
- [x] Configuration
- [ ] Connectivity
