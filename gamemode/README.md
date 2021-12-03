# GameMode-go

A lightweight (zero cycles) program to kill DWM, explorer, set a timer resolution, and disable idle before gaming. 

## CONFIGURATION:

Inside of `config.json` you can configure how gamemode behaves:

TIMERRES: Should be a number not in quotes, if you want your resolution to be 0.5: 5000, 1.0: 10000 etc..

KILL_DWM: Kills DWM, cannot be set to true while KILL_EXPLORER is set to true (KILL_DWM kills explorer)

KILL_EXPLORER: If your game does not run well with DWM killed, you can atleast kill explorer. Cannot be set to true while KILL_DWM is set to true.

DISABLE_IDLE: Disables idle while gamemode is active

## USAGE:

1. Launch your game
2. Launch gamemode
3. alt-tab to your game
4. Once finished, alt-tab back to gamemode and press any key to restore functionality.

Gamemode should be launched with NSudo (make sure it's somewhere in your path) like so:

```
nsudo -U:T -P:E -CurrentDirectory:C:\CHANGE\ME
```