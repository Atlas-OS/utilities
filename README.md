# Atlas-Utilities

Source Repo for Atlas Utilities. These utilities are free to be used, modified, and distributed in other projects, per the [License](https://github.com/Atlas-OS/Atlas-Utilities/blob/main/LICENSE)

## GameUtil-rs
![screenshot](/img/gameutil-rs.png)

A tool to automate common tasks before gaming:
  - Setting a timer resolution (5000 = 0.5ms)
  - Killing DWM (or explorer)
  - Disabling Idle
  - "Cleaning" Memory

All settings will only change once you click the Start button, other than the Clean Memory Button.

### Usage

1. Open your Game
2. Open Gameutil
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
