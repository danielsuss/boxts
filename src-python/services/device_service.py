import pyaudio
from log import server_log

async def list_audio_devices():
    server_log("Listing all available audio devices")
    
    try:
        p = pyaudio.PyAudio()
        devices = []
        
        for i in range(p.get_device_count()):
            device_info = p.get_device_info_by_index(i)
            devices.append({
                "index": i,
                "name": device_info['name'],
                "max_input_channels": device_info['maxInputChannels'],
                "max_output_channels": device_info['maxOutputChannels'],
                "default_sample_rate": device_info['defaultSampleRate'],
                "host_api": device_info['hostApi'],
                "is_input": device_info['maxInputChannels'] > 0,
                "is_output": device_info['maxOutputChannels'] > 0
            })
        
        p.terminate()
        
        # Log each device for debugging
        for device in devices:
            if device['is_output']:
                server_log(f"Output Device {device['index']}: {device['name']} (Channels: {device['max_input_channels']}, SR: {device['default_sample_rate']})")
        
        return {"status": "success", "devices": devices}
        
    except Exception as e:
        server_log(f"Error listing devices: {str(e)}")
        return {"status": "error", "message": f"Failed to list devices: {str(e)}"}