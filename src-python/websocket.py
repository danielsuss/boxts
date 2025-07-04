from fastapi import WebSocket
import asyncio
from log import server_websocket_log

# WebSocket connections for ready signals
ready_connections = set()

async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    ready_connections.add(websocket)
    
    # Send ready signal immediately when connection is established
    await signal_ready_ws()
    
    try:
        # Keep connection alive
        while True:
            await websocket.receive_text()
    except Exception:
        pass
    finally:
        ready_connections.discard(websocket)

async def signal_ready_ws():
    """Signal ready to all WebSocket connections"""
    if ready_connections:
        for connection in ready_connections.copy():
            try:
                await connection.send_text("ready")
                server_websocket_log("Ready!")
            except Exception:
                ready_connections.discard(connection)

async def signal_notification_ws(message: str):
    """Send notification message to all WebSocket connections"""
    if ready_connections:
        for connection in ready_connections.copy():
            try:
                await connection.send_text(f"notification {message}")
                server_websocket_log(f"Notification: {message}")
            except Exception:
                ready_connections.discard(connection)
    
    # Yield control to allow WebSocket message to be sent immediately
    await asyncio.sleep(0)