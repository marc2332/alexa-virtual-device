This runs a virtual Alexa device which opens up a https://github.com/marc2332/iamnothacker

**NOTE**: some code is hardcoded for now (It only works on **WSL**), but it's easy to adapt in case you wanna try it. The device runs on port 1100 by default.

### Usage

Run:
```shell
cargo run -- <IP> <DEVICE_NAME>
```
Specify the IP of the machine where the program is running and the virtual device's name (whatever you want).