from fastapi import FastAPI
from boxts_manager import BoxtsManager
from RealtimeTTS import TextToAudioStream, CoquiEngine
import pyaudio
import uvicorn
import os
from utils import setup_ffmpeg, is_production_environment
from models import SpeakRequest, TrainModelRequest, StartRequest
from config import get_output_device, get_volume
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
    
    try:
        if boxts_manager.stream is None:
            return {"status": "error", "message": "TTS not started. Use /start command first."}
        
        # Feed text to the stream
        boxts_manager.stream.feed(request.text)

        boxts_manager.stream.play_async()
        
        return {"status": "success", "message": f"Speaking: {request.text}"}
        
    except Exception as e:
        server_log(f"Error speaking text: {str(e)}")
        return {"status": "error", "message": f"Failed to speak text: {str(e)}"}

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

@app.post("/start")
async def start_tts(request: StartRequest):
    server_log(f"Starting TTS with voice: {request.voice}")
    
    try:
        # Get audio device configuration
        output_device_name = get_output_device()
        volume = get_volume()
        
        # Get output device index using pyaudio
        p = pyaudio.PyAudio()
        output_device_index = None
        
        for i in range(p.get_device_count()):
            device_info = p.get_device_info_by_index(i)
            if device_info['name'] == output_device_name:
                output_device_index = i
                break
        
        p.terminate()
        
        if output_device_index is None:
            output_device_index = 0  # Use default device
            server_log(f"Device '{output_device_name}' not found, using default device")

        # Set voices path based on environment
        if is_production_environment():
            voices_path = "./realtimetts-resources/voices"
        else:
            voices_path = "../realtimetts-resources/voices"

        # Create CoquiEngine with voice
        boxts_manager.engine = CoquiEngine(
            voice=request.voice,
            voices_path=voices_path,
            device="cuda"
        )
        
        # Create TextToAudioStream with volume
        boxts_manager.stream = TextToAudioStream(
            boxts_manager.engine,
            output_device_index=output_device_index
        )
        
        boxts_manager.stream.volume = volume

        # Feed initial text before starting async streaming (legacy pattern)
        boxts_manager.stream.feed("HOLY FUCK WE'RE BACK")
        
        # Start async streaming
        boxts_manager.stream.play_async()
        
        server_log(f"TTS started successfully with voice: {request.voice}")
        return {"status": "success", "message": f"TTS started with voice: {request.voice}"}
        
    except Exception as e:
        server_log(f"Error starting TTS: {str(e)}")
        return {"status": "error", "message": f"Failed to start TTS: {str(e)}"}


if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8000)