from pydantic import BaseModel

class SpeakRequest(BaseModel):
    text: str

class TrainModelRequest(BaseModel):
    filepath: str