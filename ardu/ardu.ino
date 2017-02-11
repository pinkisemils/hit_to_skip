#include <rgb_lcd.h>

unsigned long lasttime = 0;
unsigned long blank = 1000; // ms between closest hits

const int piezo = 4;

rgb_lcd lcd;
bool lcdstate = false;

String trackbuffer = String();

void setup() {
  lcd.begin(16, 2);
  pinMode(piezo, INPUT);
  Serial.begin(9600);
  lcd.setPWM(REG_RED, 0);
  lcd.setPWM(REG_BLUE, 0);
  lcd.setPWM(REG_GREEN, 255);
  lcd.display();
  lcd.print("Ok let's go");
  Serial.println("Listening");
}

void loop() {
  if (millis() - blank > lasttime && digitalRead(piezo)){
    lasttime = millis();
    lcdstate = true;
    lcd.setPWM(REG_RED, 255);
    lcd.setPWM(REG_GREEN, 0);
    Serial.println("S");
  }

  if (lcdstate && millis() - blank > lasttime){
    lcdstate = false;
    lcd.setPWM(REG_RED, 0);
    lcd.setPWM(REG_GREEN, 255);
    Serial.println("L");
  }

  while (Serial.available()){
    char inputchar = Serial.read();
    if (inputchar == 0x03) { // End of Text (^C)
      lcd.clear();
      lcd.print(trackbuffer.c_str());
      trackbuffer = "";  // I suspect this is a leak. There's no other documented way to clear a String other than naive
                         // looping over length() and set_at()
    } else {
      if (trackbuffer.length() >= 32){
        lcd.clear();
        lcd.print(trackbuffer.c_str());
      } else {
        trackbuffer += inputchar;
      }
    }
  }
}
