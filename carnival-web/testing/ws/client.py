import websocket
import rel


def on_open(ws):
    print("on_open")


def on_close(ws, err):
    print(f"on_close -> {err}")


def on_error(ws, error):
    print(f"on_error -> {error}")


def on_message(ws, message):
    print(f"on_message -> {message}")


if __name__ == "__main__":
    # ?? python camelCase 
    websocket.enableTrace(True)
    ws = websocket.WebSocketApp(
            "ws://127.0.0.1:3000/ws/notifications",
            on_open=on_open,
            on_message=on_message,
            on_error=on_error,
            on_close=on_close)
    ws.run_forever(dispatcher=rel, reconnect=5)
    rel.signal(2, rel.abort)
    rel.dispatch()
