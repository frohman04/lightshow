import json

START_TOP = 0
START_LEFT = 0
DELTA_X = 162.2-133.4
DELTA_Y = 54.1-25.3

WIDTH = 37
HEIGHT = 22

id = 0

diagram_json = {
    "version": 1,
    "author": "Chris Lieb",
    "editor": "wokwi",
    "parts": [
        { "type": "wokwi-arduino-nano", "id": "nano", "top": -4.8, "left": -192.5 , "attrs": {} },
    ],
    "connections": [
        [ "nano:2", "rgb0:DIN", "green", [ "h0" ] ],
        [ "nano:GND.2", "rgb0:VSS", "black", [ "v0" ] ],
        [ "nano:5V", "rgb0:VDD", "red", [ "v0" ] ],
    ],
    "dependencies": {},
}

# output LEDs
# top edge
for i in range(WIDTH):
    diagram_json["parts"].append({
        "type": "wokwi-neopixel",
        "id": f"rgb{id}",
        "top": round(START_TOP, 1),
        "left": round(START_LEFT + DELTA_X * (1 + i), 1),
        "attrs": {},
    })
    id += 1
# right edge
for i in range(HEIGHT):
    diagram_json["parts"].append({
        "type": "wokwi-neopixel",
        "id": f"rgb{id}",
        "top": round(START_TOP + DELTA_Y * (1 + i), 1),
        "left": round(START_LEFT + DELTA_X * (WIDTH + 1), 1),
        "attrs": {},
    })
    id += 1
# bottom edge
for i in range(WIDTH):
    diagram_json["parts"].append({
        "type": "wokwi-neopixel",
        "id": f"rgb{id}",
        "top": round(START_TOP + DELTA_Y * (HEIGHT + 1), 1),
        "left": round(START_LEFT + DELTA_X * (WIDTH - i), 1),
        "attrs": {},
    })
    id += 1
# left edge
for i in range(HEIGHT):
    diagram_json["parts"].append({
        "type": "wokwi-neopixel",
        "id": f"rgb{id}",
        "top": round(START_TOP + DELTA_Y * (HEIGHT - i), 1),
        "left": round(START_LEFT, 1),
        "attrs": {},
    })
    id += 1

# output connections
for i in range(id):
    diagram_json["connections"] += [
        [ f"rgb{i}:VDD", f"rgb{i + 1}:VDD", "red", [ "h0" ] ],
        [ f"rgb{i}:VSS", f"rgb{i + 1}:VSS", "black", [ "v0" ] ],
        [ f"rgb{i}:DOUT", f"rgb{i + 1}:DIN", "green", [ "h0" ] ],
    ]

print(json.dumps(diagram_json, indent=2))
    