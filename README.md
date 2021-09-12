# kiosk-usb-reset

# Compiling

This does something, maybe nothing, posting it here just it case. Probably don't run it unless you figure out you need it and in what folder to run it. Probably inside Android Studio folder. Building for android most probably needs this though. Stackoverflow knows.
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

# Permanent install

All of this assumes SElinux is disabled or set to warning-mode-only, which it is on our manufacturing sample.

The commands below remounts the system as writeable, moves the program to permanent storage and sets up a service.

* adb connect x.x.x.x
* adb root
* adb connect x.x.x.x
* adb disable-verity
* adb reboot
* adb connect x.x.x.x
* adb root
* adb connect x.x.x.x
* adb remount
* adb shell
* mount -rw,remount /system
* exit
* adb push ./kiosk-usb-reset /system/bin/kiosk-usb-reset
* adb push ./kiosk-usb-reset.rc /system/etc/init/kiosk-usb-reset.rc
* adb shell
* chown 0.0 /system/bin/kiosk-usb-reset
* chown 0.0 /system/etc/init/kiosk-usb-reset.rc
* chmod 0755 /sytem/bin/kiosk-usb-reset
* chmod 0644 /sytem/etc/init/kiosk-usb-reset.rc
* chcon u:object_r:system_file:s0 /system/etc/init/kiosk-usb-reset.rc
* exit
* adb reboot
* adb connect x.x.x.x
* adb shell
* ps -A | grpe kiosk
* dmesg | grep kiosk