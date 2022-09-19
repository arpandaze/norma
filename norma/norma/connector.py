import json

import numpy
import rel
import websocket
from bagchal import Bagchal

from .model import model


def get_best_move_pgn(bagchal: Bagchal):
    possible_moves = bagchal.get_possible_moves()

    input_vectors = bagchal.state_as_inputs(possible_moves)

    inputs = numpy.asarray(input_vectors)

    predication = model.predict_on_batch(inputs)

    best_move_index = predication.argmax()

    move = possible_moves[best_move_index]["resulting_state"].prev_move

    return Bagchal.coord_to_png_unit(*move)


def on_message(ws, msg):
    message = json.loads(msg)

    if message["type"] == 10:
        try:
            game = Bagchal.new()
            game.turn = message["game"]["turn"]
            game.goat_counter = message["game"]["goat_counter"]
            game.goat_captured = message["game"]["goat_captured"]
            game.game_history = message["game"]["game_history"]
            game.pgn = message["game"]["pgn"]

            pgn_unit = get_best_move_pgn(game)

            ws.send(
                json.dumps(
                    {
                        "type": 1,
                        "move": pgn_unit,
                        "game_id": message["game"]["game_id"],
                    }
                )
            )
        except Exception as e:
            import traceback
            print(traceback.format_exc())
            print(e)
            raise e


def on_error(*args, **kwargs):
    print(f"Error occured with args: {args}, {kwargs}")


def on_close(*args, **kwargs):
    print(f"Websocket closed with args: {args}, {kwargs}")


def on_open(*args, **kwargs):
    print(f"Websocket opened with args: {args}, {kwargs}")


def launch_executor():
    websocket.enableTrace(False)

    ws = websocket.WebSocketApp(
        "ws://localhost:8080/glory?ident=d4beab3f-9b3c-44df-ba62-d477fb33c67b",
        on_open=on_open,
        on_message=on_message,
        on_error=on_error,
        on_close=on_close,
    )

    ws.run_forever(dispatcher=rel)
    rel.signal(2, rel.abort)
    rel.dispatch()
