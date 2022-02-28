# Build and Run
## Prereqs:
- In order to cross-compile for the Raspberry Pi Zero (first-gen), you may need the appropriate compiler. On Debian-based Linux distros, you should run:
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
linker = "<Raspberry Pi tools path>/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"
```

## Build:
```
cargo build --release --target arm-unknown-linux-gnueabihf
```

The release binary will be in the `target/arm-unknown-linux-gnueabihf/release` directory.

## Run:
- Create a directory on your Raspberry Pi where you'll copy over the files.
```
cd ~/Documents && mkdir rpi-albumart
```
- Copy the binary over to the Raspberry Pi.
```
scp ./target/arm-unknown-linux-gnueabihf/release/rpi-albumart pi@<Your Pi's IP>:~/Documents/rpi-albumart
```
- Copy over the templates directory.
```
scp -r ./templates pi@<Your Pi's IP>:~/Documents/rpi-albumart
```
- On your Raspberry Pi, run the binary!
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
