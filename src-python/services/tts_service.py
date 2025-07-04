from RealtimeTTS import TextToAudioStream, CoquiEngine
from boxts_manager import BoxtsManager
from audio_devices import get_output_device_index
from config import get_volume
from environment import is_production_environment
from log import server_log
from websocket import signal_ready_ws

boxts_manager = BoxtsManager()

async def speak_text(text: str):
    server_log(f"Speaking text: {text}")
    
    try:
        if boxts_manager.stream is None:
            return {"status": "error", "message": "TTS not started. Use /start command first."}
        
        # Feed text to the stream
        boxts_manager.stream.feed(text) 
        boxts_manager.stream.play_async()
        
        return {"status": "success", "message": f"Speaking: {text}"}
        
    except Exception as e:
        server_log(f"Error speaking text: {str(e)}")
        return {"status": "error", "message": f"Failed to speak text: {str(e)}"}

async def start_tts(voice: str):
    server_log(f"Starting TTS with voice: {voice}")
    
    try:
        # Check if engine already exists
        if boxts_manager.engine is not None:
            server_log("TTS already started, try using /changevoice")
            await signal_ready_ws()
            return {"status": "error", "message": "TTS already started, try using /changevoice"}

        output_device_index = get_output_device_index()
        volume = get_volume()
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
            voice=voice,
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
        
        server_log(f"TTS started successfully with voice: {voice}")
        await signal_ready_ws()
        return {"status": "success", "message": f"TTS started with voice: {voice}"}
        
    except Exception as e:
        server_log(f"Error starting TTS: {str(e)}")
        await signal_ready_ws()
        return {"status": "error", "message": f"Failed to start TTS: {str(e)}"}

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

async def change_voice(voice: str):
    server_log(f"Changing voice to: {voice}")
    
    try:
        # Check if engine exists
        if boxts_manager.engine is None:
            await signal_ready_ws()
            return {"status": "error", "message": "TTS engine not started. Use /start command first."}
        
        # Change the voice on the existing engine
        server_log(f"Setting voice to: {voice}")
        boxts_manager.engine.set_voice(voice)

        boxts_manager.stream.feed("YOU'RE ROCKING WITH ME NOW PRETTY BOY")
        boxts_manager.stream.play_async()
        
        server_log(f"Voice successfully changed to: {voice}")
        await signal_ready_ws()
        return {"status": "success", "message": f"Voice changed to: {voice}"}
        
    except Exception as e:
        server_log(f"Error changing voice: {str(e)}")
        await signal_ready_ws()
        return {"status": "error", "message": f"Failed to change voice: {str(e)}"}

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

async def change_output_device():
    server_log(f"Changing output device")
    try:
        if boxts_manager.stream is not None:
            # Store current volume before stopping
            current_volume = boxts_manager.stream.volume
            boxts_manager.stream.stop()
            
            output_device_index = get_output_device_index()

            # Clear the stream reference
            boxts_manager.stream = None
            
            # Create new stream with updated output device
            boxts_manager.stream = TextToAudioStream(
                boxts_manager.engine,
                output_device_index=output_device_index
            )
            
            # Restore volume setting
            boxts_manager.stream.volume = current_volume
        else:
            # No TTS stream exists yet, just log the change
            server_log("No TTS stream exists yet, device will be used when TTS starts")
        
        await signal_ready_ws()
        return {"status": "success", "message": "Output device changed successfully."}
        
    except Exception as e:
        server_log(f"Error changing output device: {str(e)}")
        await signal_ready_ws()
        return {"status": "error", "message": f"Failed to change output device: {str(e)}"}

async def send_ready_signal():
    server_log("Manual ready signal requested")
    await signal_ready_ws()
    return {"status": "success", "message": "Ready signal sent"}