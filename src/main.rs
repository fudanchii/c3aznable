use esp_idf_svc::sys::{esp, esp_vfs_dev_uart_use_driver, uart_driver_install};

use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::prelude::*;

use c3aznable::char_lcd::{CharLCD, LcdPinMap};
use c3aznable::led::{LedState, RgbLed};

use std::{io, ptr};

use std::io::prelude::*;

fn main() {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    unsafe {
        esp!(uart_driver_install(0, 512, 512, 10, ptr::null_mut(), 0)).unwrap();
        esp_vfs_dev_uart_use_driver(0);
    }

    let peripherals = Peripherals::take().unwrap();
    let rgb_led = RgbLed::new(peripherals.rmt.channel0, peripherals.pins.gpio2).unwrap();

    rgb_led.turn(LedState::Off).unwrap();

    let lcd_device = CharLCD::init(LcdPinMap {
        rs: peripherals.pins.gpio0,
        en: peripherals.pins.gpio10,

        d4: peripherals.pins.gpio4,
        d5: peripherals.pins.gpio5,
        d6: peripherals.pins.gpio6,
        d7: peripherals.pins.gpio7,
    })
    .unwrap();

    let mut display = lcd::Display::new(lcd_device);
    display.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    display.display(
        lcd::DisplayMode::DisplayOn,
        lcd::DisplayCursor::CursorOn,
        lcd::DisplayBlink::BlinkOn,
    );

    loop {
        let mut input = Vec::new();

        print!("$>: ");

        use std::io::Write;
        io::stdout().flush().unwrap();

        readline(&mut input);
        if input.is_empty() {
            continue;
        };

        match String::from_utf8_lossy(&input).as_ref().trim() {
            "led:off" => {
                let _ = rgb_led.turn(LedState::Off).unwrap();
            }

            "led:red" => {
                let _ = rgb_led.turn(LedState::Red).unwrap();
            }

            "led:green" => {
                let _ = rgb_led.turn(LedState::Green).unwrap();
            }

            "led:blue" => {
                let _ = rgb_led.turn(LedState::Blue).unwrap();
            }

            "led:random" | "led:rand" | "led:rnd" => {
                let _ = rgb_led.turn(LedState::Random).unwrap();
            }

            "clr" => {
                display.clear();

                display.display(
                    lcd::DisplayMode::DisplayOn,
                    lcd::DisplayCursor::CursorOn,
                    lcd::DisplayBlink::BlinkOn,
                );
            }

            refstr if refstr.starts_with("display:") => {
                display.display(
                    lcd::DisplayMode::DisplayOn,
                    lcd::DisplayCursor::CursorOff,
                    lcd::DisplayBlink::BlinkOff,
                );

                let data = &input[8..input.len() - 1];

                for c in data {
                    display.write(*c);
                }
            }

            _ => {
                log::error!(
                    "unknown command: {}",
                    String::from_utf8_lossy(&input).as_ref().trim()
                );
                continue;
            }
        }
    }
}

fn readline(input: &mut Vec<u8>) {
    if let Err(err) = io::stdin().lock().read_until(b'\n', input) {
        match err.kind() {
            io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut | io::ErrorKind::Interrupted => {
                Ets::delay_us(100)
            }
            _ => log::error!("err : {err}"),
        }
    }
}
