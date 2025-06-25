@echo off
echo Starting TTS Server only...

REM Start Python TTS Server
env_realtimetts\Scripts\python.exe tts_server.py

echo TTS Server stopped.
pause