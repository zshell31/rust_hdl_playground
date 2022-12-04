#![feature(generic_const_exprs)]
mod multiplexer;

//use std::time::Duration;

use rust_hdl::prelude::*;

//const CLOCK_SPEED_HZ: u64 = 10_000;

use multiplexer::Mux;

#[derive(LogicBlock, Default)]
struct Blinky {
    pub enable: Signal<In, Bit>,
    pub clock: Signal<In, Clock>,
    pub led: Signal<Out, Bit>,
    mux: Mux<Bit, 1>,
    //pub pulser: Pulser,
}

// impl Default for Blinky {
//     fn default() -> Self {
//         Self {
//             enable: Default::default(),
//             clock: Default::default(),
//             led: Default::default(),
//             mux: Default::default(),
//         }
//     }
// }

impl Logic for Blinky {
    #[hdl_gen]
    fn update(&mut self) {
        self.led.next = self.mux.outsig.val();
        //self.pulser.clock.next = self.clock.val();
        //self.pulser.enable.next = true.into();
        //self.led.next = self.pulser.pulse.val();
    }
}

// #[derive(LogicBlock, Default)]
// struct SignExtender {
//     //pub clock: Signal<In, Clock>,
//     pub input: Signal<In, Bits<32>>,
//     pub output: Signal<Out, Bits<12>>,
// }

// impl Logic for SignExtender {
//     #[hdl_gen]
//     fn update(&mut self) {
//         self.output.next = self.output.val().get_bits::<Bits<12>>(20);
//         //self.ouptut.set_bits(12, self.output.val())
//     }
// }

fn main() {
    let uut = Blinky::default();
    //uut.connect_all();

    println!("{}", generate_verilog_unchecked(&uut));
}
