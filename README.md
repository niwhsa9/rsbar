# RS Bar
This is a status bar I have written for DWM with Rust. It is my first Rust project!

 My goal is for this to support some custom widgets that other DWM status bars don't, including some internet-enabled widgets. This is meant to be run as a systemd service, so it undoubtedly does not adhere to the suckless philosophy ¯\\\_(ツ)_/¯.

### Current features
- System battery
- Date time
- Discord notifications via DBus interaction

### Planned features
- Slack notify
- Integration with my IoT soil moisture system
- Monitor configuration
- Automatic timezone adjustment