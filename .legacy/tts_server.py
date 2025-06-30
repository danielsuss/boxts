from RealtimeTTS import TextToAudioStream, CoquiEngine
from flask import Flask, jsonify, request
import pyaudio

app = Flask(__name__)

@app.route('/tts', methods=['POST'])
def tts():
    data = request.json
    if not data or 'text' not in data:
        return jsonify({"error": "No text provided"}), 400

    text = data['text']
    
    stream.feed(text)
    stream.play_async()

    return jsonify({"message": "Text processed and played successfully"}), 200

if __name__ == "__main__":
    print("Finding audio device index for 'CABLE In 16ch (VB-Audio Virtual Cable)'...")

    p = pyaudio.PyAudio()
    target_index = 0

    for i in range(p.get_device_count()):
        device_info = p.get_device_info_by_index(i)
        if device_info['name'] == 'CABLE In 16 Ch (VB-Audio Virtual Cable)':
            target_index = i
            print(f"Found at index {i}")

    p.terminate()

    print("Starting TTS engine...")
    engine = CoquiEngine(voice="paarthurnax.json", device="cuda")
    stream = TextToAudioStream(engine, output_device_index=target_index) # ouput_device_index=18
    stream.feed("TTS SESSION STARTED")  
    stream.play_async()
    print("TTS engine started successfully")
    print("Starting server...")
    app.run(port=5000)