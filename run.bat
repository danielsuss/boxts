@echo off
echo Starting BoxTS Application...

REM Start Python TTS Server in background
echo Starting TTS Server...
start /B "TTS Server" env_realtimetts\Scripts\python.exe tts_server.py

REM Wait a moment for server to start
timeout /t 3 /nobreak >nul

REM Start Tauri application
echo Starting BoxTS GUI...
src-tauri\target\release\boxts.exe

echo BoxTS Application closed.
pause