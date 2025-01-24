#include <Adafruit_NeoPixel.h>
#ifdef __AVR__
  #include <avr/power.h>
#endif
#define PIN         2
#define NUMPIXELS 119
#define MSGSIZE     4

Adafruit_NeoPixel pixels(NUMPIXELS, PIN, NEO_GRB + NEO_KHZ800);
#define DELAYVAL 50

void setup() {
#if defined(__AVR_ATtiny85__) && (F_CPU == 16000000)
  clock_prescale_set(clock_div_1);
#endif
  Serial.begin(115200);
  pixels.begin();
}

void loop() {
  if (Serial.available() >= MSGSIZE) {
    byte message[MSGSIZE];
    int bytesReadLen = Serial.readBytes(message, MSGSIZE);
    if (bytesReadLen == MSGSIZE) {
      Serial.print("Setting pixel ");
      Serial.print(message[0]);
      Serial.print(" to RGB [");
      Serial.print(message[1]);
      Serial.print(", ");
      Serial.print(message[2]);
      Serial.print(", ");
      Serial.print(message[3]);
      Serial.print("]\n");
      pixels.setPixelColor(message[0], pixels.Color(message[1], message[2], message[3]));
      pixels.show();
    }
  }
}
