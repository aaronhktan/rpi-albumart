# <img src="/docs/icon/icon.png?raw=true" height="48"> Raspberry Pi Album Art

Show album art from the currently playing track according to Last.fm. Or, if you're not listening to anything, show albums listened to in the last week. Inspired by [Jason Tate's Now Playing Project](https://chorus.fm/news/now-playing-my-raspberry-pi-weekend-project/).

<img src="/docs/screenshots/art_1.png?raw=true" height="200"> <img src="/docs/screenshots/art_2.png?raw=true" height="200"> <img src="/docs/screenshots/art_3.png?raw=true" height="200"> <img src="/docs/screenshots/art_4.png?raw=true" height="200"> <img src="/docs/screenshots/stats_1.png?raw=true" height="200"> <img src="/docs/screenshots/photo_1.jpg?raw=true" height="200">

# Required Hardware
- **Raspberry Pi Zero, first-gen** (should work on other Pis as well but have not tested)
- **[Hyperpixel 4.0 Square Touch](https://shop.pimoroni.com/products/hyperpixel-4-square?variant=30138251444307)**
- **[3D-printed enclosure by Printminion](https://www.thingiverse.com/thing:4536253)**

# Build and Run
## Prereqs:
- Run the [Hyperpixel setup scripts](https://github.com/pimoroni/hyperpixel4) on the Raspberry Pi.
- In order to cross-compile for the Raspberry Pi Zero, you may need the appropriate compiler. On Debian-based Linux distros, you should run:
```
sudo apt install gcc-arm-linux-gnueabihf
```
- You will also need to download Raspberry Pi tools.
```
git clone https://github.com/raspberrypi/tools
```
- Finally, add the file `.cargo/config` with the following content:
```
[target.arm-unknown-linux-gnueabihf]
linker = "<PATH_TO_PI_TOOLS>/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"
```

## Build:
```
cargo build --release --target arm-unknown-linux-gnueabihf
```

The release binary will be in the `target/arm-unknown-linux-gnueabihf/release` directory.

## Run:
- **On the Raspberry Pi**, create a directory where you'll copy over the files.
```
cd ~/Documents && mkdir rpi-albumart
```
- **From the Linux computer where you built the binary**, copy the binary over to the Raspberry Pi.
```
scp ./target/arm-unknown-linux-gnueabihf/release/rpi-albumart pi@<PI_IP_ADDRESS_HERE>:~/Documents/rpi-albumart
```
- Copy over the templates directory.
```
scp -r ./templates pi@<PI_IP_ADDRESS_HERE>:~/Documents/rpi-albumart
```
- Copy over the static directory.
```
scp -r ./static pi@<PI_IP_ADDRESS_HERE>:~/Documents/rpi-albumart
```
- **On your Raspberry Pi**, run the binary!
```
./rpi-albumart
```
- If everything worked, you should see the following:
```
🛰  Routes:
   >> (hello) GET /?<user>&<key>
   >> (FileServer: static/) GET /static/<path..> [10]
👾 Catchers:
   >> (not_found) 404
📡 Fairings:
   >> Templating (ignite, liftoff)
   >> Shield (liftoff, response, singleton)
🚀 Rocket has launched from http://0.0.0.0:8000
```
- Visit `localhost:8000/?user=<LAST_FM_USERNAME>&key=<LAST_FM_API_KEY>` in the browser!

## Autostart

- Add a script called `launch.sh` at the root of this repository, with the following contents.
```
#!/bin/bash

# Working directory must be root of this repository,
# so that static files can be served.
# Put the absolute path! This script may be called
# as another user.
cd <PATH_TO_REPOSITORY>
./rpi-albumart
```
- To run this app on startup, create an autostart script on the Raspberry Pi.
```
cd ~
mkdir -p .config/lxsession/LXDE-pi
vim .config/lxsession/LXDE-pi/autostart
```
- Paste the following contents into the autostart script.
```
@lxpanel --profile LXDE-pi
@pcmanfm --desktop --profile LXDE-pi
# disable blank screen
@xset s off
@xset -dpms
@xset s noblank
# launch binary
@sh <PATH_TO_REPOSITORY>/launch.sh
# wait for server to start
@sleep 30s
# launch chrome
/usr/bin/chromium-browser --kiosk --disable-restore-session-state http://localhost:8000/?user=<LAST_FM_USERNAME>&key=<LAST_FM_API_KEY>
# disable showing cursor
@unclutter -idle 0
@xscreensaver -no-splash
```
- Reboot, and then check the logs on the Raspberry Pi. You should see the same success message somewhere in the logs as when the binary was first run.
```
tail -n 200 ~/.cache/lxsession/LXDE-pi/run.log
```
