from fastapi import FastAPI
from boxts_manager.py import BoxtsManager
from RealtimeTTS import TextToAudioStream, CoquiEngine
import pyaudio
import uvicorn

app = FastAPI(title="Boxts TTS Server", version="0.1.0")
boxts_manager = BoxtsManager()

@app.get("/")
async def root():
    return {"message": "Boxts TTS Server is running"}

@app.post("/speak")
async def speak(text: str):
    print(f"Received text: {text}")
    return {"status": "success", "message": f"Processing: {text}"}

@app.post("/trainmodel")
async def trainmodel(filepath: str):
    print(f"Received training file path: {filepath}")
    return {"status": "success", "message": f"Training file received: {filepath}"}

if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8000)