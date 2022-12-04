use rust_hdl::prelude::*;

pub struct MuxParam<const N: usize> {}

pub trait AllowableMuxParam {}

impl AllowableMuxParam for MuxParam<1> {}

impl AllowableMuxParam for MuxParam<2> {}

impl AllowableMuxParam for MuxParam<3> {}

impl AllowableMuxParam for MuxParam<4> {}

impl AllowableMuxParam for MuxParam<5> {}

pub struct Mux<D: Synth, const A: usize>
where
    [Signal<In, D>; 1 << A]: Sized,
    MuxParam<A>: AllowableMuxParam,
{
    pub input_lines: [Signal<In, D>; 1 << A],
    pub select: Signal<In, Bits<A>>,
    pub outsig: Signal<Out, D>,
}

impl<D: Synth, const A: usize> Default for Mux<D, A>
where
    [Signal<In, D>; 1 << A]: Sized,
    MuxParam<A>: AllowableMuxParam,
{
    fn default() -> Self {
        Self {
            input_lines: unsafe {
                use std::mem::MaybeUninit;

                let mut result = MaybeUninit::<[Signal<In, D>; 1 << A]>::uninit();
                let dest = result.as_mut_ptr() as *mut Signal<In, D>;
                for _ in 0..(1 << A) {
                    dest.write(Default::default());
                }

                result.assume_init()
            },
            select: Default::default(),
            outsig: Default::default(),
        }
    }
}

impl<D: Synth, const A: usize> Block for Mux<D, A>
where
    [Signal<In, D>; 1 << A]: Sized,
    MuxParam<A>: AllowableMuxParam,
{
    fn connect_all(&mut self) {
        self.connect();
        self.input_lines.connect_all();
        self.select.connect_all();
        self.outsig.connect_all();
    }

    fn update_all(&mut self) {
        self.update();
        self.input_lines.update_all();
        self.select.update_all();
        self.outsig.update_all();
    }

    fn has_changed(&self) -> bool {
        self.input_lines.has_changed() || self.select.has_changed() || self.outsig.has_changed()
    }

    fn accept(&self, name: &str, probe: &mut dyn Probe) {
        probe.visit_start_scope(name, self);
        self.input_lines.accept("input_lines", probe);
        self.select.accept("select", probe);
        self.outsig.accept("outsig", probe);
        probe.visit_end_scope(name, self);
    }
}

impl<D: Synth, const A: usize> Logic for Mux<D, A>
where
    [Signal<In, D>; 1 << A]: Sized,
    MuxParam<A>: AllowableMuxParam,
{
    #[hdl_gen]
    fn update(&mut self) {
        for i in 0..(1 << A) {
            if self.select.val().index() == i {
                self.outsig.next = self.input_lines[i].val();
            }
        }
    }
}
