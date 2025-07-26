# Hardware Connection Blueprint

## Component Connections

### ESP32-WROOM-32 Pinout
```
ESP32 Pin    | Component          | Purpose
-------------|-------------------|------------------
GPIO32       | Microphone AO     | ADC input
GPIO25       | Passive Buzzer S  | PWM output
GPIO33       | LED (optional)    | Status indicator
GND          | All components    | Common ground
3.3V         | Microphone VCC    | Microphone power
5V           | Buzzer VCC        | Buzzer power (if needed)
```

## Detailed Connections

### Elegoo "Big Sound" Microphone Module
- **VCC** → ESP32 **3.3V** (red wire)
- **GND** → ESP32 **GND** (black wire)  
- **AO** → ESP32 **GPIO32** (signal wire)
- **DO** → Not connected (digital output not used)

### Elegoo Passive Buzzer Module
- **VCC** → ESP32 **3.3V** or **5V** (red wire)*
- **GND** → ESP32 **GND** (black wire)
- **S** → ESP32 **GPIO25** (signal wire)

*Note: Try 3.3V first; use 5V if buzzer volume is too low

### Optional Status LED
- **Anode (+)** → ESP32 **GPIO33** through 220Ω resistor
- **Cathode (-)** → ESP32 **GND**

### Test Setup (Active Buzzer)
- **VCC** → External **5V** power source
- **GND** → Common ground with ESP32
- **S** → Connect to 5V to activate (for testing)

## Physical Layout Suggestions
1. Place ESP32 on breadboard center
2. Mount microphone module on left side
3. Mount buzzer module on right side
4. Keep microphone and buzzer physically separated to minimize feedback
5. Use short, direct wire connections
6. Ensure stable power connections

## Troubleshooting Notes
- If microphone readings are too sensitive, adjust the onboard potentiometer
- If buzzer is too quiet, try connecting VCC to 5V instead of 3.3V
- Ensure all grounds are connected to the same rail
- Keep analog and digital sections separated when possible