# ![logo][]
All in one ADB tool that has multiple FL (Family Link) bypasses. 

[logo]: images/ParentUnlockLogo.png

## Usage

T is tested
U is untested

1. `pun chronolink <hrs>` for chronolink exploit (time travel to a time where you didnt have downtime with hrs) (T)
2. `pun dns [hostname]` (hostname is optional) for DNS exploit (T)
3. `pun totp <secret> [ts]` for TOTP exploit (needs shared secret) (timestamp is optional) (U)
4. `pun removefl` to remove FL and mi security center (U)
5. `pun 2space` for second space exploit (hyperos) (U)
6. `pun removegms` to remove gms.supervision (U)
7. `pun fixbedtime` to fix the bedtime screen (U)
8. `pun forcehl` to force the hyper launcher (hyperos?) (U)
9. `pun 2user` to make a second user and then switch the current user to the second one (U)
10. `pun reset` to reset every exploit (almost, not yet done) (U)

If you find anymore exploits/bypasses by ADB, or you see a typo or a bug, please PR.

## How to actually use it
>[!WARNING]
>You need a computer with ADB installed and a device with developer options and usb debugging enabled for this to work. Developer options can be enabled on the parent familyLink app.

1. Connect your device to your computer using a usb cable or wifi
2. Download latest ver. from releases
3. Open terminal where your binary is
4. Run (replace <exploit> with exploit from usage):
   1) Linux & MacOS: `./pun <exploit`
   2) Windows: `pun <exploit>`

## Troubleshooting

### "Device not found" error
Turn on dev options and usb debugging
Reconnect cable
Reconnect with device using adb if your using wifi

### Cannot run program on Linux/MacOS
1. Open terminal where your binary is
2. Type `chmod +x pun` and press enter

## Credits
[@rifting](https://github.com/rifting) and [anti-link](https://github.com/anti-link) for chronolink, dns, and totp exploits

[u/adrainnpropp](https://www.reddit.com/user/Adriannpropp/) and [r/familylink](https://www.reddit.com/r/familylink/) for removefl, 2space, removegms, fixbedtime, forcehl exploits
