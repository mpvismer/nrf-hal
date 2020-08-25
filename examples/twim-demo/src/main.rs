#![no_std]
#![no_main]

#[allow(unused_imports)]
use embedded_hal::blocking::i2c::{Read, Write};
use embedded_hal::digital::v2::InputPin;
use {
    core::{
        panic::PanicInfo,
        sync::atomic::{compiler_fence, Ordering},
    },
    hal::{
        gpio::{p0::Parts, Input, Pin, PullUp},
        gpiote::Gpiote,
        pac::TWIM0,
        twim::*,
    },
    nrf52840_hal as hal,
    rtic::cyccnt::U32Ext,
    rtt_target::{rprintln, rtt_init_print},
};

#[rtic::app(device = crate::hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        twim: Twim<TWIM0>,
        gpiote: Gpiote,
        btn1: Pin<Input<PullUp>>,
        btn2: Pin<Input<PullUp>>,
        btn3: Pin<Input<PullUp>>,
        btn4: Pin<Input<PullUp>>,
    }

    #[init]
    fn init(mut ctx: init::Context) -> init::LateResources {
        let _clocks = hal::clocks::Clocks::new(ctx.device.CLOCK).enable_ext_hfosc();
        ctx.core.DCB.enable_trace();
        ctx.core.DWT.enable_cycle_counter();
        rtt_init_print!();

        let p0 = Parts::new(ctx.device.P0);
        let scl = p0.p0_30.into_floating_input().degrade();
        let sda = p0.p0_31.into_floating_input().degrade();
        let btn1 = p0.p0_11.into_pullup_input().degrade();
        let btn2 = p0.p0_12.into_pullup_input().degrade();
        let btn3 = p0.p0_24.into_pullup_input().degrade();
        let btn4 = p0.p0_25.into_pullup_input().degrade();

        let gpiote = Gpiote::new(ctx.device.GPIOTE);
        gpiote.port().input_pin(&btn1).low();
        gpiote.port().input_pin(&btn2).low();
        gpiote.port().input_pin(&btn3).low();
        gpiote.port().input_pin(&btn4).low();
        gpiote.port().enable_interrupt();

        let twim = Twim::new(ctx.device.TWIM0, Pins { scl, sda }, Frequency::K100);

        init::LateResources {
            twim,
            gpiote,
            btn1,
            btn2,
            btn3,
            btn4,
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        rprintln!("Press button 1 to READ from addr 0x1A");
        rprintln!("Press button 2 to WRITE to addr 0x1A");
        rprintln!("Press button 3 to READ from addr 0x1B");
        rprintln!("Press button 4 to WRITE from addr 0x1B");
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = GPIOTE, resources = [gpiote], schedule = [debounce])]
    fn on_gpiote(ctx: on_gpiote::Context) {
        ctx.resources.gpiote.reset_events();
        ctx.schedule.debounce(ctx.start + 3_000_000.cycles()).ok();
    }

    #[task(resources = [twim, gpiote, btn1, btn2, btn3, btn4])]
    fn debounce(ctx: debounce::Context) {
        let twim = ctx.resources.twim;
        if ctx.resources.btn1.is_low().unwrap() {
            rprintln!("\nREAD from address 0x1A");
            let rx_buf = &mut [0; 8][..];
            let res = twim.read(0x1A, rx_buf);
            rprintln!("Result: {:?}\n{:?}", res, rx_buf);
        }
        if ctx.resources.btn2.is_low().unwrap() {
            rprintln!("\nWRITE to address 0x1A");
            let tx_buf = [1, 2, 3, 4, 5, 6, 7, 8];
            let res = twim.write(0x1A, &tx_buf[..]);
            rprintln!("Result: {:?}\n{:?}", res, tx_buf);
        }
        if ctx.resources.btn3.is_low().unwrap() {
            rprintln!("\nREAD from address 0x1B");
            let rx_buf = &mut [0; 4][..];
            let res = twim.read(0x1B, rx_buf);
            rprintln!("Result: {:?}\n{:?}", res, rx_buf);
        }
        if ctx.resources.btn4.is_low().unwrap() {
            rprintln!("\nWRITE to address 0x1B");
            let tx_buf = [9, 10, 11, 12];
            let res = twim.write(0x1B, &tx_buf[..]);
            rprintln!("Result: {:?}\n{:?}", res, tx_buf);
        }
    }

    extern "C" {
        fn SWI0_EGU0();
    }
};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    rprintln!("{}", info);
    loop {
        compiler_fence(Ordering::SeqCst);
    }
}
