#include <Adafruit_NeoPixel.h>
#ifdef __AVR__
  #include <avr/power.h>
#endif
#define PIN         2
#define NUMPIXELS 119
#define MSGSIZE     3
#define DEBUG       1

#define LOW         0
#define HIGH      255

Adafruit_NeoPixel pixels(NUMPIXELS, PIN, NEO_GRB + NEO_KHZ800);
#define DELAYVAL 50

void setup() {
#if defined(__AVR_ATtiny85__) && (F_CPU == 16000000)
  clock_prescale_set(clock_div_1);
#endif
  Serial.begin(115200);
  pixels.begin();
}

byte buffer1[MSGSIZE];
byte buffer2[MSGSIZE];
byte* currBuffer = buffer1;
byte* prevBuffer = buffer2;
byte* tempBuffer;
int i = 0;

void loop() {
  if (Serial.available() >= MSGSIZE) {
    int bytesReadLen = Serial.readBytes(currBuffer, MSGSIZE);
    if (bytesReadLen == MSGSIZE) {
      if (prevBuffer[0] == LOW && prevBuffer[1] == HIGH && prevBuffer[2] == LOW
          && currBuffer[0] == HIGH && currBuffer[1] == LOW && currBuffer[2] == HIGH) {
        if (DEBUG) {
          Serial.println("Found sentinel");
        }
        i = 0;
      } else {
        if (DEBUG) {
          Serial.print("Setting pixel ");
          Serial.print(i);
          Serial.print(" to RGB [");
          Serial.print(currBuffer[0]);
          Serial.print(", ");
          Serial.print(currBuffer[1]);
          Serial.print(", ");
          Serial.print(currBuffer[2]);
          Serial.print("]\n");
        }
        pixels.setPixelColor(i++, pixels.Color(currBuffer[0], currBuffer[1], currBuffer[2]));
        pixels.show();
      }

      // swap buffers
      tempBuffer = currBuffer;
      currBuffer = prevBuffer;
      prevBuffer = tempBuffer;
    }
  }
}
