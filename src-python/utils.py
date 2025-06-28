import os
import pydub

def setup_ffmpeg():
    """
    Configure pydub to use bundled FFmpeg in production environment.
    In development, developers handle their own FFmpeg installation.
    Should be called at server startup before any audio processing.
    """
    app_dir = os.path.dirname(os.path.abspath(__file__))
    
    # Check if we're in production (bundled app with _up_ structure)
    is_production = "_up_" in app_dir
    
    if not is_production:
        print("Development mode: Using system FFmpeg")
        return False
    
    # Production mode - setup bundled FFmpeg
    # FFmpeg is bundled at same level as src-python, so go up one directory
    parent_dir = os.path.dirname(app_dir)
    ffmpeg_dir = os.path.join(parent_dir, "ffmpeg-resources")
    ffmpeg_exe = os.path.join(ffmpeg_dir, "ffmpeg.exe") 
    ffprobe_exe = os.path.join(ffmpeg_dir, "ffprobe.exe")
    
    if os.path.exists(ffmpeg_exe):
        pydub.AudioSegment.converter = ffmpeg_exe
        pydub.AudioSegment.ffmpeg = ffmpeg_exe
        if os.path.exists(ffprobe_exe):
            pydub.AudioSegment.ffprobe = ffprobe_exe
        print(f"Production mode: Using bundled FFmpeg: {ffmpeg_exe}")
        return True
    else:
        print("Production mode: Bundled FFmpeg not found, using system FFmpeg")
        return False