use esp_idf_hal::{
    adc::{Adc, AdcChannelDriver, AdcDriver, Attenuation},
    gpio::{Gpio32, Gpio33, Gpio25, PinDriver, OutputPin},
    ledc::{LedcDriver, LedcTimerDriver, config::TimerConfig},
    peripherals::Peripherals,
    delay::FreeRtos,
};
use esp_idf_sys as _;

const SAMPLE_RATE_MS: u32 = 1; // 1ms = 1kHz sampling
const DETECTION_THRESHOLD: u16 = 2000; // ADC threshold for beep detection
const MIN_BEEP_DURATION_MS: u32 = 50; // Minimum beep length
const MAX_BEEP_DURATION_MS: u32 = 3000; // Maximum beep length
const REPLAY_FREQUENCY_HZ: u32 = 300; // Lower frequency for elderly hearing
const MIC_DISABLE_DELAY_MS: u32 = 50; // Delay before replaying

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_hal::sys::esp_log_level_set("*", esp_idf_sys::esp_log_level_t_ESP_LOG_INFO);

    let peripherals = Peripherals::take().unwrap();
    
    // Initialize ADC for microphone (GPIO32)
    let mut adc = AdcDriver::new(peripherals.adc1, &esp_idf_hal::adc::config::Config::new())?;
    let mut mic_pin = AdcChannelDriver::<_, Attenuation::DB_11>::new(peripherals.pins.gpio32)?;
    
    // Initialize PWM for passive buzzer (GPIO25)
    let timer_config = TimerConfig::default().frequency(REPLAY_FREQUENCY_HZ.into());
    let timer = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config)?;
    let mut buzzer = LedcDriver::new(peripherals.ledc.channel0, &timer, peripherals.pins.gpio25)?;
    
    // Status LED (GPIO33) - optional for debugging
    let mut status_led = PinDriver::output(peripherals.pins.gpio33)?;
    
    println!("Beep Relay Device Started");
    
    let mut beep_detected = false;
    let mut beep_start_time = 0u32;
    let mut mic_enabled = true;
    let mut mic_disable_until = 0u32;
    
    loop {
        let current_time = esp_idf_hal::sys::esp_timer_get_time() as u32 / 1000; // Convert to ms
        
        // Re-enable microphone after delay
        if !mic_enabled && current_time >= mic_disable_until {
            mic_enabled = true;
            println!("Microphone re-enabled");
        }
        
        if mic_enabled {
            // Read microphone ADC value
            let mic_value = adc.read(&mut mic_pin)?;
            
            // Check if we're detecting a beep
            let signal_detected = mic_value > DETECTION_THRESHOLD;
            
            if signal_detected && !beep_detected {
                // Start of new beep
                beep_detected = true;
                beep_start_time = current_time;
                status_led.set_high()?;
                println!("Beep detected! ADC: {}", mic_value);
                
            } else if !signal_detected && beep_detected {
                // End of beep
                let beep_duration = current_time - beep_start_time;
                beep_detected = false;
                status_led.set_low()?;
                
                // Validate beep duration
                if beep_duration >= MIN_BEEP_DURATION_MS && beep_duration <= MAX_BEEP_DURATION_MS {
                    println!("Valid beep ended. Duration: {}ms", beep_duration);
                    
                    // Disable microphone temporarily
                    mic_enabled = false;
                    mic_disable_until = current_time + MIC_DISABLE_DELAY_MS + beep_duration;
                    
                    // Wait brief delay before replaying
                    FreeRtos::delay_ms(MIC_DISABLE_DELAY_MS);
                    
                    // Replay the beep at lower frequency
                    replay_beep(&mut buzzer, beep_duration)?;
                } else {
                    println!("Beep too short/long: {}ms - ignored", beep_duration);
                }
            }
        }
        
        FreeRtos::delay_ms(SAMPLE_RATE_MS);
    }
}

fn replay_beep(buzzer: &mut LedcDriver, duration_ms: u32) -> anyhow::Result<()> {
    println!("Replaying beep for {}ms at {}Hz", duration_ms, REPLAY_FREQUENCY_HZ);
    
    // Start PWM at 50% duty cycle
    let max_duty = buzzer.get_max_duty();
    buzzer.set_duty(max_duty / 2)?;
    
    // Keep buzzer on for the detected duration
    FreeRtos::delay_ms(duration_ms);
    
    // Turn off buzzer
    buzzer.set_duty(0)?;
    
    println!("Beep replay completed");
    Ok(())
}