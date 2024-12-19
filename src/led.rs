use esp_idf_svc::eventloop::{
    EspBackgroundEventLoop, EspBackgroundSubscription, EspEvent, EspEventDeserializer,
    EspEventPostData, EspEventSerializer, EspEventSource,
};

use esp_idf_svc::hal::{delay, gpio::OutputPin, peripheral::Peripheral, rmt};
use esp_idf_svc::sys::{esp_random, EspError};

use smart_leds::{gamma, SmartLedsWrite, RGB8};
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrbw32;
use ws2812_esp32_rmt_driver::LedPixelEsp32Rmt;

use std::ffi::CStr;

const NUM_PIXELS: usize = 1;

#[derive(Copy, Clone, Debug)]
pub enum LedState {
    Off,
    Red,
    Green,
    Blue,
    Random,
    RGB(u8, u8, u8),
}

unsafe impl EspEventSource for LedState {
    fn source() -> Option<&'static CStr> {
        Some(c"c3Az:led")
    }
}

impl EspEventDeserializer for LedState {
    type Data<'a> = LedState;

    fn deserialize<'a>(data: &EspEvent<'a>) -> Self::Data<'a> {
        unsafe { *data.as_payload::<LedState>() }
    }
}

impl EspEventSerializer for LedState {
    type Data<'a> = LedState;

    fn serialize<F, R>(ev: &Self::Data<'_>, f: F) -> R
    where
        F: FnOnce(&EspEventPostData) -> R,
    {
        f(unsafe { &EspEventPostData::new(Self::source().unwrap(), Self::event_id(), ev) })
    }
}

pub struct RgbLed {
    evloop: EspBackgroundEventLoop,
    _subs: EspBackgroundSubscription<'static>,
}

impl RgbLed {
    pub fn new(
        channel: rmt::CHANNEL0,
        pin: impl Peripheral<P = impl OutputPin> + 'static,
    ) -> Result<RgbLed, EspError> {
        let evloop = EspBackgroundEventLoop::new(&Default::default())?;
        let mut ws2812 = LedPixelEsp32Rmt::<RGB8, LedPixelColorGrbw32>::new(channel, pin).unwrap();

        macro_rules! write_rgb {
            ($r:expr, $g:expr, $b:expr) => {{
                let pixels = std::iter::repeat(RGB8 {
                    r: $r,
                    g: $g,
                    b: $b,
                })
                .take(NUM_PIXELS);
                ws2812.write(gamma(pixels)).unwrap();
            }};
        }

        let result = evloop.subscribe::<LedState, _>(move |msg| match msg {
            LedState::Off => write_rgb!(0, 0, 0),
            LedState::Red => write_rgb!(255, 0, 0),
            LedState::Green => write_rgb!(0, 255, 0),
            LedState::Blue => write_rgb!(0, 0, 255),
            LedState::Random => write_rgb!(
                unsafe { esp_random() } as u8,
                unsafe { esp_random() } as u8,
                unsafe { esp_random() } as u8
            ),
            LedState::RGB(r, g, b) => write_rgb!(r, g, b),
        });

        result.map(|_subs| RgbLed { evloop, _subs })
    }

    pub fn turn(&self, state: LedState) -> Result<bool, EspError> {
        self.evloop.post::<LedState>(&state, delay::NON_BLOCK)
    }
}
