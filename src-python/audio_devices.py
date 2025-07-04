import pyaudio
from config import get_output_device
from log import server_log

def get_output_device_index():
    output_device_name = get_output_device()

    # Prevent problematic VB Cable setup
    if output_device_name in ["CABLE Input (VB-Audio Virtual Cable)", "CABLE In 16ch (VB-Audio Virtual Cable)"]:
        output_device_name = "CABLE Input (VB-Audio Virtual C"
        server_log(f"Avoiding VB Cable problems by defaulting to {output_device_name}")

    # Get output device index using pyaudio
    p = pyaudio.PyAudio()
    output_device_index = None
    
    for i in range(p.get_device_count()):
        device_info = p.get_device_info_by_index(i)
        if device_info['name'] == output_device_name and device_info['maxOutputChannels'] > 0:
            output_device_index = i
            break
    
    p.terminate()
    
    if output_device_index is None:
        output_device_index = 0  # Use default device
        server_log(f"Device '{output_device_name}' not found, using default device")

    return output_device_index