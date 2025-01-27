#include <Adafruit_NeoPixel.h>
#include <PacketSerial.h>
#include <CRC16.h>
#ifdef __AVR__
  #include <avr/power.h>
#endif

#define PIN         2
#define NUMPIXELS 118
#define MSGSIZE     3
#define DEBUG

#define LOW       " "
#define HIGH      "~"

Adafruit_NeoPixel pixels(NUMPIXELS, PIN, NEO_GRB + NEO_KHZ800);
#define DELAYVAL 50

PacketSerial myPacketSerial;

void setup() {
#if defined(__AVR_ATtiny85__) && (F_CPU == 16000000)
  clock_prescale_set(clock_div_1);
#endif
  myPacketSerial.begin(115200);
  myPacketSerial.setPacketHandler(&onPacketReceived);
  pixels.begin();
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

const size_t pixelDataStartI = 2;
const CRC16 crc = CRC16(
    CRC16_ARC_POLYNOME,
    CRC16_ARC_INITIAL,
    CRC16_ARC_XOR_OUT,
    CRC16_ARC_REV_IN,
    CRC16_ARC_REV_OUT
);

void onPacketReceived(const uint8_t* buffer, size_t size) {
#ifdef DEBUG
  Serial.print("Received packet (size ");
  Serial.print(size);
  Serial.print("): ");
  for (int i = 0; i < size; i++) {
    Serial.print(String(buffer[i], HEX));
  }
  Serial.print("\n");
#endif

  if (size < 4) {
    // illegal packet size, ignore
#ifdef DEBUG
    Serial.println("ERROR: Received packet with too small size, discarding");
#endif
  } else {
    crc.restart();

    for (int i = 0; i < size - 2; i++) {
      crc.add(buffer[i]);
    }
    uint16_t expectedCrc = crc.calc();
    uint16_t actualCrc = ((uint16_t)buffer[size-2] << 8) | buffer[size-1];
    if (actualCrc != expectedCrc) {
      // CRC mismatch, discard packet
#ifdef DEBUG
      Serial.print("ERROR: Received packet mismatched CRC, discarding (expected ");
      Serial.print(String(expectedCrc, HEX));
      Serial.print(", got ");
      Serial.print(String(actualCrc, HEX));
      Serial.print(")\n");
#endif
    } else {
      uint8_t pixelOffset = buffer[0];
      uint8_t numPixels = buffer[1];

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
    }
  }
}
