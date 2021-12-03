# GameMode-go

A lightweight (zero cycles) program to kill DWM, explorer, set a timer resolution, and disable idle before gaming. 

## CONFIGURATION:

Inside of `config.json` you can configure how gamemode behaves:

TIMERRES: Should be a number not in quotes, if you want your resolution to be 0.5: 5000, 1.0: 10000 etc..

KILL_DWM: Kills DWM, cannot be set to true while KILL_EXPLORER is set to true (KILL_DWM kills explorer)

KILL_EXPLORER: If your game does not run well with DWM killed, you can atleast kill explorer. Cannot be set to true while KILL_DWM is set to true.

DISABLE_IDLE: Disables idle while gamemode is active

## USAGE:

This should be launched with NSudo like so:

nsudo -U:T -P:E -CurrentDirectory:C:\CHANGE\ME