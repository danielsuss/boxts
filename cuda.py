import subprocess
import os
import re

def check_cuda_version():
    """Check CUDA version using nvcc command."""
    try:
        result = subprocess.run(["nvcc", "--version"], capture_output=True, text=True)
        version = re.search(r"release (\d+\.\d+)", result.stdout)
        if version:
            return f"CUDA Version: {version.group(1)}"
        else:
            return "CUDA not found or version could not be detected."
    except FileNotFoundError:
        return "CUDA is not installed or nvcc is not in the PATH."

def check_cudnn_version():
    """Check cuDNN version from the header file cudnn_version.h."""
    cudnn_paths = [
        r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v12.8\include\cudnn_version.h",  # Windows path pattern
        "/usr/local/cuda/include/cudnn_version.h"  # Linux path
    ]
    for path in cudnn_paths:
        if os.path.exists(path):
            with open(path, "r") as f:
                content = f.read()
                major = re.search(r"#define CUDNN_MAJOR (\d+)", content)
                minor = re.search(r"#define CUDNN_MINOR (\d+)", content)
                patch = re.search(r"#define CUDNN_PATCHLEVEL (\d+)", content)
                if major and minor and patch:
                    return f"cuDNN Version: {major.group(1)}.{minor.group(1)}.{patch.group(1)}"
    return "cuDNN is not installed or cudnn_version.h could not be found."

if __name__ == "__main__":
    print(check_cuda_version())
    print(check_cudnn_version())
