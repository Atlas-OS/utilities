# Atlas-Utilities

Source Repo for Atlas Utilities. These utilities are free to be used, modified, and distributed in other projects, per the [License](https://github.com/Atlas-OS/Atlas-Utilities/blob/main/LICENSE)

## filepicker

Simple file picker in Go using [go-common-file-dialog](https://github.com/harry1453/go-common-file-dialog)

This returns the path of a file e.g. `C:\Windows\System32\notepad.exe`

## MultiChoice

Simple multiple choice dialog in Go

This is originally written by [spddl](https://github.com/spddl), full credits to them.

To use it you can run:

```
multichoice.exe "This is a title" "This is a prompt" "this;is;four;options"
```

## GameUtil-go

A lightweight (zero cycles) program to kill DWM, explorer, set a timer resolution, and disable idle before gaming. 

### Configuration

Inside of `config.json` you can configure how gamemode behaves:

`TIMERRES`: Should be a number not in quotes, if you want your resolution to be 0.5: 5000, 1.0: 10000 etc..

`KILL_DWM`: Kills DWM, cannot be set to true while KILL_EXPLORER is set to true (KILL_DWM kills explorer)

`KILL_EXPLORER`: If your game does not run well with DWM killed, you can atleast kill explorer. Cannot be set to true while KILL_DWM is set to true.

`DISABLE_IDLE`: Disables idle while gamemode is active

### Usage

1. Launch your game
2. Click start in GameUtil
3. alt-tab to your game
4. Once finished, alt-tab back to GameUtil and press "Restore" to restore functionality.

Gamemode should be launched with [NSudo](https://github.com/M2Team/NSudo/releases/latest) (make sure it's somewhere in your path) like so:

```
nsudo -U:T -P:E -CurrentDirectory:C:\CHANGE\ME gameutil.exe
```