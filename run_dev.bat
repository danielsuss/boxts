@echo off
echo Starting BoxTS Development Environment...

REM Start Python TTS Server in background
echo Starting TTS Server...
start /B "TTS Server" env_realtimetts\Scripts\python.exe tts_server.py

REM Wait a moment for server to start
timeout /t 3 /nobreak >nul

REM Start Tauri development mode
echo Starting BoxTS in development mode...
npm run tauri dev

echo BoxTS Development session ended.
pause