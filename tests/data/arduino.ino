// 23 lines 13 code 6 comments 4 blanks

int led = 13;

void setup() {
    // setup println()
    Serial.begin(155200);
    // Init LED pin
    pinMode(led, OUTPUT);
}

/**
 * Blink the LED
*/
void loop() {
    Serial.println("LED ON!");
    digitalWrite(led, HIGH);
    delay(1000);

    Serial.println("LED OFF!");
    digitalWrite(led, LOW);
    delay(1000);
}
