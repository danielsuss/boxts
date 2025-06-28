from pydantic import BaseModel

class TextRequest(BaseModel):
    text: str

class TrainModelRequest(BaseModel):
    filepath: str