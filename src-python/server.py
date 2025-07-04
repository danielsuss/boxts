from fastapi import FastAPI, WebSocket
import uvicorn
from environment import setup_ffmpeg
from routes import router
from websocket import websocket_endpoint

# Setup FFmpeg for audio processing
setup_ffmpeg()

app = FastAPI(title="Boxts TTS Server", version="0.1.0")

# Include routes
app.include_router(router)

# WebSocket endpoint
@app.websocket("/ws")
async def websocket_handler(websocket: WebSocket):
    await websocket_endpoint(websocket)

if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8000)