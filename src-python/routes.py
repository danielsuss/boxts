from fastapi import APIRouter
from models import SpeakRequest, TrainModelRequest, VoiceRequest
from services.tts_service import (
    speak_text, start_tts, stop_tts, change_voice, 
    update_volume, change_output_device, send_ready_signal
)
from services.voice_service import clone_voice_from_file
from services.device_service import list_audio_devices

router = APIRouter()

@router.get("/")
async def root():
    return {"message": "Boxts TTS Server is running"}

@router.post("/speak")
async def speak(request: SpeakRequest):
    return await speak_text(request.text)

@router.post("/clonevoice")
async def clonevoice(request: TrainModelRequest):
    return await clone_voice_from_file(request.filepath)

@router.post("/outputdevice")
async def outputdevice():
    return await change_output_device()

@router.post("/start")
async def start_tts_endpoint(request: VoiceRequest):
    return await start_tts(request.voice)

@router.post("/volume")
async def volume_endpoint():
    return await update_volume()

@router.post("/listdevices")
async def list_devices():
    return await list_audio_devices()

@router.post("/stop")
async def stop_tts_endpoint():
    return await stop_tts()

@router.post("/changevoice")
async def change_voice_endpoint(request: VoiceRequest):
    return await change_voice(request.voice)

@router.post("/ready")
async def ready():
    return await send_ready_signal()