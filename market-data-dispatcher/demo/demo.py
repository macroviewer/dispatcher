import asyncio
import websockets

async def listen():
    uri = "ws://127.0.0.1:8080"
    async with websockets.connect(uri) as websocket:
        await websocket.send("Hello, Server!")
        while True:
            message = await websocket.recv()
            print(f"收到消息: {message}")

asyncio.run(listen())
