#!/bin/sh
./gradlew build
./gradlew installDebug
adb shell am start -n me.appventure.mobileapp/me.appventure.mobileapp.MainActivity
