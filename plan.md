STACK

- React TS
- shadcn - or just css
- Tauri v2
- Python
- FastAPI
- SQLite

FRONT END

- simple text box in consistent position on screen

BACK END

- Tauri invoke commands to send message back and forth with server
- Tauri global hotkey handling

SERVER

- Central main orchestrating operation
- ServerManager
- InputParser ? (/ commands) - maybe in tauri side
- TTSManager
- DBManager

27/06/25

FRONTEND

- implement item selector in front end
- split main into components? - zustand?

BACKEND

- save configuration - boxts.conf.toml

SERVER

/trainmodel
/model
/start
/stop

![alt text](setup_plan.png)

/START

flow:

- /start
- tauri invoke command get_voices (from realtimetts-resources/voices (need to set filepath for both dev and prod env))
  - returns rotated list of voice.json files in the target dir, rotated to the voice saved under the voice field under [tts] in boxts.conf.toml
- item selector in main.tsx lets user choose from voices returned by get_voices
- on voice chosen, start_command callback triggered in commands.rs
- start_command:
  - calls set_voice from config.rs (saving the voice to boxts.conf.toml)
  - calls send_start_request from bridge.rs (requesting the /start route with the voice file)

in python:

- /start route
- get output device index - get_output_device combined with pyaudio to match device name to index (can see how this is done in .legacy)
- create CoquiEngine using voice, output_device_index, "cuda" - assign to boxts_manager.engine
- create TextToAudioStream using boxts_manager.engine, volume from get_volume (config.py) - assign to boxts_manager.stream
- start streaming audio (can see how this is done in .legacy)

30/06/25

FRONTEND

- rotated list for /outputdevice
- change item selector font styling - current selection cyan, everything italic?
- update command handler so it cycles through all potential matches upon tab press
- press escape to cancel two stage command
- improve ux when waiting on commands to run / server to start
- can still send command if it is half way through a correct one

BACKEND

- /stop route
- /start should kill any instance of a stream and coqui engine if exists

GENERAL

- debug virtual audio cable not working

02/07/25

FRONT END

- Loading... while waiting for
  - server to start/restart
  - clonevoice
  - tts starting

BUILD TEST

- have output as log aswell as stdout - possible?
- loading not working for clonevoice
