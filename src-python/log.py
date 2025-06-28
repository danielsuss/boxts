SERVER_STRING = "\033[95mSERVER\033[37m:\033[0m   "

def server_log(message: str):
    print(f"{SERVER_STRING}{message}")