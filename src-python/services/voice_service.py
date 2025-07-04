from voice_cloning import clone_voice
from log import server_log
from websocket import signal_ready_ws, signal_notification_ws

async def clone_voice_from_file(filepath: str):
    server_log(f"Cloning voice from file: {filepath}")
    
    try:
        embedding_path, voice_name = await clone_voice(filepath)
        server_log(f"Voice cloning completed: {embedding_path}")
        await signal_ready_ws()
        await signal_notification_ws("vocal patch created!")
        return {"status": "success", "message": f"Voice cloned successfully: {voice_name}"}
        
    except Exception as e:
        server_log(f"Error cloning voice: {str(e)}")
        await signal_ready_ws()
        await signal_notification_ws("error cloning voice")
        return {"status": "error", "message": f"Failed to clone voice: {str(e)}"}