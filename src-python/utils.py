import os
import pydub
from log import server_log

def is_production_environment():
    app_dir = os.path.dirname(os.path.abspath(__file__))
    return "_up_" in app_dir

def setup_ffmpeg():
    if not is_production_environment():
        server_log("Development mode: Using system FFmpeg")
        return False
    
    app_dir = os.path.dirname(os.path.abspath(__file__))
    parent_dir = os.path.dirname(app_dir)
    ffmpeg_dir = os.path.join(parent_dir, "ffmpeg-resources")
    ffmpeg_exe = os.path.join(ffmpeg_dir, "ffmpeg.exe") 
    ffprobe_exe = os.path.join(ffmpeg_dir, "ffprobe.exe")
    
    if os.path.exists(ffmpeg_exe):
        pydub.AudioSegment.converter = ffmpeg_exe
        pydub.AudioSegment.ffmpeg = ffmpeg_exe
        if os.path.exists(ffprobe_exe):
            pydub.AudioSegment.ffprobe = ffprobe_exe
        server_log(f"Production mode: Using bundled FFmpeg: {ffmpeg_exe}")
        return True
    else:
        server_log("Production mode: Bundled FFmpeg not found, using system FFmpeg")
        return False