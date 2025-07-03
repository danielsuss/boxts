from fastapi import FastAPI, WebSocket
from boxts_manager import BoxtsManager
from RealtimeTTS import TextToAudioStream, CoquiEngine
import pyaudio
import uvicorn
import os
from utils import setup_ffmpeg, is_production_environment, signal_ready
from models import SpeakRequest, TrainModelRequest, VoiceRequest
from config import get_output_device, get_volume
from log import server_log, server_websocket_log
from tts_utils import clone_voice

# Setup FFmpeg for audio processing
setup_ffmpeg()

boxts_manager = BoxtsManager()

# WebSocket connections for ready signals
ready_connections = set()

app = FastAPI(title="Boxts TTS Server", version="0.1.0")
 
@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    ready_connections.add(websocket)
    
    # Send ready signal immediately when connection is established
    await signal_ready_ws()
    
    try:
        # Keep connection alive
        while True:
            await websocket.receive_text()
    except Exception:
        pass
    finally:
        ready_connections.discard(websocket)

async def signal_ready_ws():
    """Signal ready to all WebSocket connections"""
    if ready_connections:
        for connection in ready_connections.copy():
            try:
                await connection.send_text("ready")
                server_websocket_log("Ready!")
            except Exception:
                ready_connections.discard(connection)

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
        await signal_ready_ws()
        return {"status": "success", "message": f"Voice cloned successfully: {voice_name}"}
        
    except Exception as e:
        server_log(f"Error cloning voice: {str(e)}")
        await signal_ready_ws()
        return {"status": "error", "message": f"Failed to clone voice: {str(e)}"}

@app.post("/start")
async def start_tts(request: VoiceRequest):
    server_log(f"Starting TTS with voice: {request.voice}")
    
    try:
        # Check if engine already exists
        if boxts_manager.engine is not None:
            server_log("TTS already started, try using /changevoice")
            await signal_ready_ws()
            return {"status": "error", "message": "TTS already started, try using /changevoice"}
        # Get audio device configuration
        output_device_name = get_output_device()

        # Prevent problematic VB Cable setup
        if output_device_name == "CABLE Input (VB-Audio Virtual Cable)" or "CABLE In 16ch (VB-Audio Virtual Cable)":
            output_device_name = "CABLE Input (VB-Audio Virtual C"
            server_log(f"Avoiding VB Cable problems by defaulting to {output_device_name}")

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

        server_log(f"Selected output device index: {output_device_index}")

        # Set voices path based on environment
        if is_production_environment():
            voices_path = "./realtimetts-resources/voices"
            models_path = "./realtimetts-resources/models"
        else:
            voices_path = "../realtimetts-resources/voices"
            models_path = "../realtimetts-resources/models"

        # Create CoquiEngine with voice
        boxts_manager.engine = CoquiEngine(
            voice=request.voice,
            voices_path=voices_path,
            local_models_path=models_path,
            specific_model="v2.0.3",
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
        await signal_ready_ws()
        return {"status": "success", "message": f"TTS started with voice: {request.voice}"}
        
    except Exception as e:
        server_log(f"Error starting TTS: {str(e)}")
        await signal_ready_ws()
        return {"status": "error", "message": f"Failed to start TTS: {str(e)}"}

@app.post("/volume")
async def update_volume():
    server_log("Updating volume from config")
    
    try:
        if boxts_manager.stream is None:
            return {"status": "error", "message": "TTS not started. Use /start command first."}
        
        # Get volume from config and update stream
        volume = get_volume()
        boxts_manager.stream.volume = volume
        
        server_log(f"Volume updated to: {volume}")
        return {"status": "success", "message": f"Volume updated to: {volume}"}
        
    except Exception as e:
        server_log(f"Error updating volume: {str(e)}")
        return {"status": "error", "message": f"Failed to update volume: {str(e)}"}

@app.post("/listdevices")
async def list_devices():
    server_log("Listing all available audio devices")
    
    try:
        p = pyaudio.PyAudio()
        devices = []
        
        for i in range(p.get_device_count()):
            device_info = p.get_device_info_by_index(i)
            devices.append({
                "index": i,
                "name": device_info['name'],
                "max_input_channels": device_info['maxInputChannels'],
                "max_output_channels": device_info['maxOutputChannels'],
                "default_sample_rate": device_info['defaultSampleRate'],
                "host_api": device_info['hostApi'],
                "is_input": device_info['maxInputChannels'] > 0,
                "is_output": device_info['maxOutputChannels'] > 0
            })
        
        p.terminate()
        
        # Log each device for debugging
        for device in devices:
            if device['is_output']:
                server_log(f"Output Device {device['index']}: {device['name']} (Channels: {device['max_input_channels']}, SR: {device['default_sample_rate']})")
        
        return {"status": "success", "devices": devices}
        
    except Exception as e:
        server_log(f"Error listing devices: {str(e)}")
        return {"status": "error", "message": f"Failed to list devices: {str(e)}"}

@app.post("/stop")
async def stop_tts():
    server_log("Stopping TTS and cleaning up resources")
    
    try:
        # Stop and cleanup stream if it exists
        if boxts_manager.stream is not None:
            server_log("Stopping TextToAudioStream...")
            boxts_manager.stream.stop()
            
            # Shutdown the engine if it exists
            if boxts_manager.stream.engine is not None:
                server_log("Shutting down CoquiEngine...")
                boxts_manager.stream.engine.shutdown()
            
            # Clear the stream reference
            boxts_manager.stream = None
            server_log("TextToAudioStream cleaned up")
        
        # Clear engine reference
        if boxts_manager.engine is not None:
            server_log("Clearing engine reference...")
            boxts_manager.engine = None

        server_log("TTS stopped and resources cleaned up successfully")
        await signal_ready_ws()
        return {"status": "success", "message": "TTS stopped and resources cleaned up"}
        
    except Exception as e:
        server_log(f"Error stopping TTS: {str(e)}")
        await signal_ready_ws()
        return {"status": "error", "message": f"Failed to stop TTS: {str(e)}"}

@app.post("/changevoice")
async def change_voice(request: VoiceRequest):
    server_log(f"Changing voice to: {request.voice}")
    
    try:
        # Check if engine exists
        if boxts_manager.engine is None:
            await signal_ready_ws()
            return {"status": "error", "message": "TTS engine not started. Use /start command first."}
        
        # Change the voice on the existing engine
        server_log(f"Setting voice to: {request.voice}")
        boxts_manager.engine.set_voice(request.voice)

        boxts_manager.stream.feed("YOU'RE ROCKING WITH ME NOW PRETTY BOY")
        
        boxts_manager.stream.play_async()
        
        server_log(f"Voice successfully changed to: {request.voice}")
        await signal_ready_ws()
        return {"status": "success", "message": f"Voice changed to: {request.voice}"}
        
    except Exception as e:
        server_log(f"Error changing voice: {str(e)}")
        await signal_ready_ws()
        return {"status": "error", "message": f"Failed to change voice: {str(e)}"}

@app.post("/ready")
async def ready():
    server_log("Manual ready signal requested")
    await signal_ready_ws()
    return {"status": "success", "message": "Ready signal sent"}


if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8000)
    signal_ready_ws()