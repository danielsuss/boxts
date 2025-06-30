from pydantic import BaseModel

class SpeakRequest(BaseModel):
    text: str

class TrainModelRequest(BaseModel):
    filepath: str

class StartRequest(BaseModel):
    voice: str