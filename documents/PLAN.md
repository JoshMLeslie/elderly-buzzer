# Beep Relay Device - Project Plan

## Overview
Create a microphone-to-buzzer relay device that detects high-frequency beeps (≥1kHz) from household appliances and replays them at lower frequencies suitable for elderly users with hearing loss.

## Hardware Components
- ESP32-WROOM-32 development board
- Elegoo "Big Sound" microphone breakout board
- Elegoo passive buzzer breakout board
- Active buzzer (for testing only)
- Breadboard and jumper wires

## Core Functionality
1. **Audio Detection**: Continuously sample microphone input via ADC
2. **Frequency Analysis**: Use simple threshold detection for frequencies ≥1kHz
3. **Duration Tracking**: Measure beep duration while signal exceeds threshold
4. **Tone Generation**: Replay detected beeps at 200-500Hz using PWM
5. **Self-Prevention**: Brief microphone disable during buzzer output

## Technical Approach
- **Language**: Rust using esp-idf-hal crate
- **ADC Sampling**: 8kHz sample rate for 1kHz+ detection
- **Processing**: Real-time threshold-based detection (no FFT needed)
- **PWM Output**: Generate lower frequency tones on passive buzzer
- **Timing**: Track beep duration, add 50ms delay before replay

## Development Phases
1. **Hardware Setup**: Wire components per blueprint
2. **Basic I/O**: Test ADC reading and PWM buzzer output
3. **Detection Logic**: Implement threshold-based frequency detection
4. **Integration**: Combine detection with tone generation
5. **Testing**: Validate with active buzzer test setup
6. **Optimization**: Fine-tune thresholds and timing

## Success Criteria
- Reliably detects 1kHz+ beeps from test buzzer
- Generates clear lower-frequency replica tones
- No false triggers or self-triggering
- Responsive operation with minimal delay