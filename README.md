# smaRSt-Fan

A Linux service written for controlling a PWM capable fan with PID control logic and smart connectivity (probably using Home Assistant).

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

## ToDo

- [x] Basics: Control fan with PWM, read out actual RPM
- [x] PID Control
- [x] Service
- [ ] Configuration
- [ ] Connectivity
