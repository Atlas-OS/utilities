# Atlas-Utilities

These utilities are free to be used, modified, and distributed in other projects. The license is [GPLv3](https://github.com/Atlas-OS/Atlas-Utilities/blob/main/LICENSE).

**Read the Contribution Guidelines:** https://docs.atlasos.net/contributions

## GameUtil-rs
![screenshot](/img/gameutil-rs.png)

A tool to automate common tasks before gaming:
  - Setting a timer resolution
  - Killing Desktop Window Manager (or explorer)
  - Disabling CPU Idle states
  - "Cleaning" Memory (Button or Press F4)

All settings will only change once you click the Start button, other than the Clean Memory Button (which also has a hotkey to clean if ingame, F4).

### Usage

1. Open your Game
2. Open Gameutil **with [NSudo](https://nsudo.m2team.org)**
3. Configure settings and click start
4. Play!
5. Once finished, alt-tab to GameUtil and click Restore

## filepicker

Simple file picker in Go using [go-common-file-dialog](https://github.com/harry1453/go-common-file-dialog)

This returns the path of a file e.g. `C:\Windows\System32\notepad.exe`

An alternative version written in rust is now [available](/filepicker-rs).

## MultiChoice

Simple multiple choice dialog in Go

This is originally written by [spddl](https://github.com/spddl), full credits to them.

To use it you can run:

```
multichoice.exe "This is a title" "This is a prompt" "this;is;four;options"
```

## sfc-fix

Preinstall Utility which modifies `HKLM\Components` to comply with `sfc /scannow` by removing manifests/hashes for stripped components.

List may be expanded, for now only tested on 1803.

### Usage

1. Mount an image with NTLite or DISM
2. Open sfc-fix
3. Go to MOUNTPOINT\Windows\System32\config and select the COMPONENTS hive
4. Done! sfc-fix will now process the registry entries
