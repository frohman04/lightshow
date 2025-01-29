#include <Adafruit_NeoPixel.h>
#include <PacketSerial.h>
#include <CRC16.h>
#ifdef __AVR__
  #include <avr/power.h>
#endif

// #define PIN         2
// #define NUMPIXELS 118
#define MSGSIZE     3
#define DEBUG

#define INST_INIT     0
#define INST_SET_LEDS 1

Adafruit_NeoPixel pixels;
bool isInitialized = false;

PacketSerial myPacketSerial;

void setup() {
#if defined(__AVR_ATtiny85__) && (F_CPU == 16000000)
  clock_prescale_set(clock_div_1);
#endif
  myPacketSerial.begin(115200);
  myPacketSerial.setPacketHandler(&onPacketReceived);
  Serial.println("Startup complete");
#ifdef DEBUG
  Serial.println("Debug mode activated");
#endif
}

void loop() {
  // The PacketSerial::update() method attempts to read in any incoming serial
  // data and emits received and decoded packets via the packet handler
  // function specified by the user in the void setup() function.
  //
  // The PacketSerial::update() method should be called once per loop(). Failure
  // to call the PacketSerial::update() frequently enough may result in buffer
  // serial overflows.
  myPacketSerial.update();
}

const size_t pixelDataStartI = 3;
const CRC16 crc = CRC16(
    CRC16_ARC_POLYNOME,
    CRC16_ARC_INITIAL,
    CRC16_ARC_XOR_OUT,
    CRC16_ARC_REV_IN,
    CRC16_ARC_REV_OUT
);

bool isCrcValid(const uint8_t* buffer, size_t size) {
  if (size < 3) {
    // illegal packet size, ignore
#ifdef DEBUG
    Serial.println("ERROR: Received packet with too small size, discarding");
#endif

    return false;
  }

  crc.restart();

  for (int i = 0; i < size - 2; i++) {
    crc.add(buffer[i]);
  }
  uint16_t expectedCrc = crc.calc();
  uint16_t actualCrc = ((uint16_t)buffer[size-2] << 8) | buffer[size-1];

  if (actualCrc == expectedCrc) {
    return true;
  } else {
    // CRC mismatch, discard packet
#ifdef DEBUG
    Serial.print("ERROR: Received packet mismatched CRC, discarding (expected ");
    Serial.print(String(expectedCrc, HEX));
    Serial.print(", got ");
    Serial.print(String(actualCrc, HEX));
    Serial.print(")\n");
#endif

    return false;
  } 
}

void onPacketReceived(const uint8_t* buffer, size_t size) {
#ifdef DEBUG
  Serial.print("DEBUG: Received packet (size ");
  Serial.print(size);
  Serial.print("): ");
  for (int i = 0; i < size; i++) {
    Serial.print(String(buffer[i], HEX));
  }
  Serial.print("\n");
#endif

  if (isCrcValid(buffer, size)) {
    uint8_t instruction = buffer[0];

    if (instruction != INST_INIT && !isInitialized) {
      // must initialize the LED strip before any other instructions can be executed
#ifdef DEBUG
      Serial.print("ERROR: Must send Init instruction before executing ");
      Serial.print(String(instruction, HEX));
      Serial.print("\n");
#endif
      return;
    }

    if (instruction == INST_INIT) {
      if (size != 5) {
        // invalid size for init instruction
#ifdef DEBUG
        Serial.print("ERROR: Invalid message size for Init message (expected 5, got ");
        Serial.print(size);
        Serial.print(")\n");
#endif
        return;
      }

      uint8_t numPixels = buffer[1];
      uint8_t pin = buffer[2];

#ifdef DEBUG
        Serial.print("DEBUG: Initializing ");
        Serial.print(numPixels);
        Serial.print(" pixels attached to pin ");
        Serial.print(pin);
        Serial.print("\n");
#endif

      pixels = new Adafruit_NeoPixel(numPixels, pin, NEO_GRB + NEO_KHZ800);
      pixels.begin();

      isInitialized = true;
    } else if (instruction == INST_SET_LEDS) {
      if (size < 5) {
        // invalid size for SetLeds instruction
#ifdef DEBUG
        Serial.print("ERROR: Invalid message size for SetLeds message (requires at least 5, got ");
        Serial.print(size);
        Serial.print(")\n");
#endif
        return;
      }

      uint8_t pixelOffset = buffer[1];
      uint8_t numPixels = buffer[2];

      uint8_t expectedSize = 5 + numPixels * MSGSIZE;
      if (size != expectedSize) {
        // invalid size for SetLeds instruction
#ifdef DEBUG
        Serial.print("ERROR: Invalid message size for SetLeds message (expected ");
        Serial.print(expectedSize);
        Serial.print(", got ");
        Serial.print(size);
        Serial.print(")\n");
#endif
        return;
      }

      for (int pixelI = 0; pixelI < numPixels; pixelI++) {
        size_t baseAddr = pixelDataStartI + MSGSIZE * pixelI;

#ifdef DEBUG
        Serial.print("DEBUG: Setting pixel ");
        Serial.print(pixelI + pixelOffset);
        Serial.print(" to RGB [");
        Serial.print(String(buffer[baseAddr + 0], HEX));
        Serial.print(", ");
        Serial.print(String(buffer[baseAddr + 1], HEX));
        Serial.print(", ");
        Serial.print(String(buffer[baseAddr + 2], HEX));
        Serial.print("]\n");
#endif

        pixels.setPixelColor(
          pixelI + pixelOffset,
          pixels.Color(
            buffer[baseAddr + 0],
            buffer[baseAddr + 1],
            buffer[baseAddr + 2]
          )
        );
      }
      pixels.show();
    } else {
      // unknown instruction, discard packet
#ifdef DEBUG
      Serial.print("ERROR: Received packet with unknown instruction, discarding (go ");
      Serial.print(String(instruction, HEX));
      Serial.print(")\n");
#endif
    }
  }
}
