import pyaudio
from config import get_output_device
from log import server_log

def get_output_device_index():
    full_device_name = get_output_device()

    # Get all available devices using pyaudio
    p = pyaudio.PyAudio()
    matching_devices = []
    
    for i in range(p.get_device_count()):
        device_info = p.get_device_info_by_index(i)
        device_name = device_info['name']
        
        # Only consider output devices and check if device name is substring of full name
        if device_info['maxOutputChannels'] > 0 and device_name in full_device_name:
            matching_devices.append({
                'index': i,
                'name': device_name,
                'length': len(device_name)
            })
    
    p.terminate()
    
    if matching_devices:
        # Find device with shortest name (most likely to work with RealtimeTTS)
        shortest_device = min(matching_devices, key=lambda d: d['length'])
        server_log(f"Selected device '{shortest_device['name']}' (shortest match for '{full_device_name}')")
        return shortest_device['index']
    else:
        # No matching devices found, use default
        server_log(f"No matching devices found for '{full_device_name}', using default device")
        return 0