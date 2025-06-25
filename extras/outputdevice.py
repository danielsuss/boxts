import pyaudio

p = pyaudio.PyAudio()

print("Available audio devices:")
for i in range(p.get_device_count()):
    device_info = p.get_device_info_by_index(i)
    print(f"Index {i}: {device_info['name']} (Output Channels: {device_info['maxOutputChannels']})")

p.terminate()
