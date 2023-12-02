# pastesim
`pastesim` is a simple tool that takes contents of clipboard and simulates keyboard input. It was build to bypass applications that don't allow pasting.

## Requirements:
- `wl-copy`
- `notify-send` - used to notify about unimplemented characters
- uinput device has path `/dev/uinput`

## Build and install
```
cargo build --release
cd target/release
sudo install pastesim /usr/local/bin
```

## Usage example
To Bind it to `Shift + V` in Hyprland, add the following line to the configuration file  `.config/hypr/hyprland.conf`
```
bind = $mainMod SHIFT, V, exec, pastesim
```

## Modifying
You can easily modify this project for your environment:
- Need other characters? - [uinput Keyboard events](https://docs.rs/uinput/latest/uinput/event/keyboard/enum.Key.html)
- X11? - just replace wl-copy
- Don't want notifications? - just delete one command

## Notes
- I disabled default features because I don't use systemd and my uinput device has path `/dev/uinput`, if your path is `/dev/misc/uinput` then you should enable default features by replacing in `Cargo.toml`:
```
uinput = { version = "0.1.3", default-features = false }
```
with
```
uinput = "0.1.3"
```
- `thread::sleep(Duration::from_secs(1));` I don't know if it has to be that long, you can add comment if you know more about it
- `thread::sleep(Duration::from_millis(1));` I've added it, because for longer strings it didn't work
