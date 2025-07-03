# boxts

A floating text-to-speech overlay application that converts your typed text into speech.

## How it works

- Press `Alt+Enter` anywhere on your system to open the text input overlay
- Type your text and press Enter
- The text is sent to a Python backend that converts it to speech using RealtimeTTS with Coqui TTS
- The window automatically hides after submission or when it loses focus

## Features

- Global hotkey activation (Alt+Enter)
- Transparent, non-intrusive floating window
- Custom voice cloning support
- Auto-hide functionality
- System tray integration

## Commands

All commands start with `/` and can be typed in the text input. Commands that require selection will show an item selector with arrow keys for navigation.

### Window Positioning

- `/center` - Center window on current monitor
- `/topleft` - Move window to top-left corner
- `/topright` - Move window to top-right corner
- `/bottomleft` - Move window to bottom-left corner
- `/bottomright` - Move window to bottom-right corner
- `/nextmonitor` - Switch to next available monitor

### Audio Configuration

- `/outputdevice` - Select audio output device
- `/volume` - Set TTS volume
- `/listdevices` - List available audio devices in console

### Voice Management

- `/start` - Start TTS with voice selection
- `/stop` - Stop TTS and clean up resources
- `/changevoice` - Change to different voice
- `/clonevoice` - Clone voice from audio file (opens file dialog)

### Application Settings

- `/lostfocus` - Configure window behavior when focus is lost (hide/show)
- `/resetconfig` - Reset all settings to defaults
- `/restartserver` - Restart the Python TTS backend
- `/exit` - Close the application

### System Commands

- `/ready` - Send manual ready signal to backend

Built with Tauri v2, React TypeScript, and Python.
