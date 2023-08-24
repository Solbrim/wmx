#### GUI!
See CLI branch to use cli instead.


#### CLI command with 2 comamnds to manipulate windows mixer

1. wmx mute <exe path contains... e.g. firefox> <time period>
2. wmx eq -- equalizies all program values to the current device volume

wmx --h -- use it, clap is 330kb of the exe size to provide a help framework. 


either way -- copy it

give credit? idgaf

this hits the window api, gets the default output device, and runs 2 command against it (mute for a period of time, and set all sessions for that device to 100% (aka 100% of that devices volume -> 100% of 7 for me)

