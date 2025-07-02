SERVER_STRING = "\033[95mSERVER\033[37m:\033[0m   "
SERVER_WS_STRING = "\033[95mSERVER\033[0m \033[91mW\033[37m:\033[0m "

def server_log(message: str):
    print(f"{SERVER_STRING}{message}")

def server_websocket_log(message: str):
    print(f"{SERVER_WS_STRING}{message}")