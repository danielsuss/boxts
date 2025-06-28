from fastapi import FastAPI
from boxts_manager import BoxtsManager
from RealtimeTTS import TextToAudioStream, CoquiEngine
import pyaudio
import uvicorn
from utils import setup_ffmpeg
from models import TextRequest, TrainModelRequest
from log import server_log

# Setup FFmpeg for audio processing
setup_ffmpeg()

app = FastAPI(title="Boxts TTS Server", version="0.1.0")
boxts_manager = BoxtsManager()

@app.get("/")
async def root():
    return {"message": "Boxts TTS Server is running"}

@app.post("/speak")
async def speak(request: TextRequest):
    server_log(f"Speaking text: {request.text}")
    return {"status": "success", "message": f"Processing: {request.text}"}

@app.post("/trainmodel")
async def trainmodel(request: TrainModelRequest):
    server_log(f"Training model with file: {request.filepath}")
    boxts_manager.engine = CoquiEngine(voice=request.filepath, device="cuda")
    return {"status": "success", "message": f"Training file received: {request.filepath}"}


if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8000)