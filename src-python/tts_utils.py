# Functions extracted from RealtimeTTS CoquiEngine:
# - download_file() (lines 966-978)
# - download_xtts_model() (lines 981-1010) 
# - load_xtts_model() (adapted from load_model() lines 510-557)
# - compute_speaker_embeddings() (adapted from get_conditioning_latents() lines 343-498)

import torch
import json
import os
import requests
from tqdm import tqdm
from TTS.config import load_config
from TTS.tts.models import setup_model as setup_tts_model
import logging
from utils import is_production_environment
from log import server_log, SERVER_STRING

def download_file(url, destination):
    response = requests.get(url, stream=True)
    total_size_in_bytes = int(response.headers.get("content-length", 0))
    block_size = 1024

    progress_bar = tqdm(total=total_size_in_bytes, unit="iB", unit_scale=True, desc=f"{SERVER_STRING}")

    with open(destination, "wb") as file:
        for data in response.iter_content(block_size):
            progress_bar.update(len(data))
            file.write(data)

    progress_bar.close()

def download_xtts_model(model_version="v2.0.2", models_base_path=None):
    if not models_base_path:
        models_base_path = "./models"
    
    model_folder = os.path.join(models_base_path, model_version)
    os.makedirs(model_folder, exist_ok=True)

    files = {
        "config.json": f"https://huggingface.co/coqui/XTTS-v2/raw/{model_version}/config.json",
        "model.pth": f"https://huggingface.co/coqui/XTTS-v2/resolve/{model_version}/model.pth?download=true",
        "vocab.json": f"https://huggingface.co/coqui/XTTS-v2/raw/{model_version}/vocab.json",
        "speakers_xtts.pth": f"https://huggingface.co/coqui/XTTS-v2/resolve/{model_version}/speakers_xtts.pth",
    }

    for file_name, url in files.items():
        file_path = os.path.join(model_folder, file_name)
        if not os.path.exists(file_path):
            server_log(f"Downloading {file_name} to {file_path}...")
            download_file(url, file_path)
            server_log(f"{file_name} downloaded successfully.")
        else:
            server_log(f"{file_name} exists in {file_path} (no download).")

    return model_folder

def load_xtts_model(model_path):
    device = torch.device(
        "cuda" if torch.cuda.is_available() else "cpu"
    )
    
    config = load_config(os.path.join(model_path, "config.json"))
    model = setup_tts_model(config)
    
    model.load_checkpoint(
        config,
        checkpoint_dir=model_path,
        checkpoint_path=None,
        vocab_path=None,
        eval=True,
        use_deepspeed=False,
    )
    model.to(device)
    
    return model

def compute_speaker_embeddings(model, audio_file_path, voice_name, voices_path):
    gpt_cond_latent, speaker_embedding = model.get_conditioning_latents(
        audio_path=audio_file_path, 
        gpt_cond_len=30, 
        max_ref_length=60
    )

    latents = {
        "gpt_cond_latent": gpt_cond_latent.cpu().squeeze().half().tolist(),
        "speaker_embedding": speaker_embedding.cpu().squeeze().half().tolist(),
    }

    embedding_file = os.path.join(voices_path, f"{voice_name}.json")
    with open(embedding_file, "w") as f:
        json.dump(latents, f)

    return embedding_file

def clone_voice(audio_file_path):
    if is_production_environment():
        models_path = "./realtimetts-resources/models"
        voices_path = "./realtimetts-resources/voices"
    else:
        models_path = "../realtimetts-resources/models"
        voices_path = "../realtimetts-resources/voices"
    
    os.makedirs(models_path, exist_ok=True)
    os.makedirs(voices_path, exist_ok=True)
    
    model_path = download_xtts_model("v2.0.2", models_path)
    model = load_xtts_model(model_path)
    
    voice_name = os.path.splitext(os.path.basename(audio_file_path))[0]
    embedding_path = compute_speaker_embeddings(
        model, audio_file_path, voice_name, voices_path
    )
    
    return embedding_path, voice_name