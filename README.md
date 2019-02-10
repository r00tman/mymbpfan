# mymbpfan
`mymbpfan` is a Linux fan speed controller for MacBookPro12,1 that considers both applesmc and coretemp readings.

Most of custom fan controllers use just coretemp, which eventually results in overheating and thermal throttling.

## Getting Started

```shell
$ git clone https://github.com/r00tman/mymbpfan
$ cd mymbpfan && cargo install --path . --force
```

The binary will be placed at `~/.cargo/bin/mymbpfan`. After that you can add it to your init system, e.g., as a systemd unit.

## Compatibility

Code is tested only with MacBookPro12,1. But it needs just paths to applesmc and to coretemp. So, if you change them accordingly, this project should work well.

Also, since MBP12,1 has just one fan, you might need to update `set_manual` and `set_speed` functions, if you need to control two fans.
