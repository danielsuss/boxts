# boxts

A floating text-to-speech overlay application that converts your typed text into speech using AI voice synthesis.

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

Built with Tauri v2, React TypeScript, and Python.
