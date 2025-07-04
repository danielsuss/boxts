import os
import tomllib
from environment import is_production_environment

def get_config_path():
    if is_production_environment():
        return "./boxts.conf.toml"
    else:
        return "../boxts.conf.toml"

def get_output_device():
    try:
        with open(get_config_path(), "rb") as f:
            config = tomllib.load(f)
            return config.get("tts", {}).get("output_device", "Default")
    except (FileNotFoundError, tomllib.TOMLDecodeError):
        return "Default"

def get_volume():
    try:
        with open(get_config_path(), "rb") as f:
            config = tomllib.load(f)
            return config.get("tts", {}).get("volume", 0.5)
    except (FileNotFoundError, tomllib.TOMLDecodeError):
        return 0.5