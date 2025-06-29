from fastapi import FastAPI
from boxts_manager import BoxtsManager
from RealtimeTTS import TextToAudioStream, CoquiEngine
import pyaudio
import uvicorn
import os
from utils import setup_ffmpeg
from models import SpeakRequest, TrainModelRequest
from log import server_log
from tts_utils import clone_voice

# Setup FFmpeg for audio processing
setup_ffmpeg()

app = FastAPI(title="Boxts TTS Server", version="0.1.0")
boxts_manager = BoxtsManager()

@app.get("/")
async def root():
    return {"message": "Boxts TTS Server is running"}

@app.post("/speak")
async def speak(request: SpeakRequest):
    server_log(f"Speaking text: {request.text}")
    return {"status": "success", "message": f"Processing: {request.text}"}

@app.post("/clonevoice")
async def clonevoice(request: TrainModelRequest):
    server_log(f"Cloning voice from file: {request.filepath}")
    
    try:
        embedding_path, voice_name = clone_voice(request.filepath)
        server_log(f"Voice cloning completed: {embedding_path}")
        return {"status": "success", "message": f"Voice cloned successfully: {voice_name}"}
        
    except Exception as e:
        server_log(f"Error cloning voice: {str(e)}")
        return {"status": "error", "message": f"Failed to clone voice: {str(e)}"}


if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8000)