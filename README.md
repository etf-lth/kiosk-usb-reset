# kiosk-usb-reset

# Compiling

This does something, maybe nothing, posting it here just it case. Probably don't run it unless you figure out you need it and in what folder to run it. Probably inside Android Studio.
```android-ndk-r10e/build/tools/make-standalone-toolchain.sh --platform=android-18 --toolchain=aarch64-linux-android-clang3.6 --install-dir=android-18-toolchain --ndk-dir=android-ndk-r10e/ --arch=arm```

Install Android target platform:
```
rustup target add aarch64-linux-android
```

Then compile for Android:
```bash
cargo build --target=aarch64-linux-android
```

# Running

After compiling for Android you need to move it to the tablet and execute. Files cannot execute in SD card so move it to local fs.

1. adb push kiosk-usb-reset /sdcard/Download
2. adb shell
3. su
4. cp /sdcard/Download/kiosk-usb-reset /data/local/tmp/
5. cd /data/local/tmp
6. ./kiosk-usb-reset
