use pgrx::guc::{GucContext, GucFlags, GucRegistry, GucSetting};

static PROBES: GucSetting<i32> = GucSetting::<i32>::new(10);
static EPSILON: GucSetting<f64> = GucSetting::<f64>::new(1.9);

pub unsafe fn init() {
    GucRegistry::define_int_guc(
        "vchordrq.probes",
        "`probes` argument of vchordrq.",
        "`probes` argument of vchordrq.",
        &PROBES,
        1,
        u16::MAX as _,
        GucContext::Userset,
        GucFlags::default(),
    );
    GucRegistry::define_float_guc(
        "vchordrq.epsilon",
        "`epsilon` argument of vchordrq.",
        "`epsilon` argument of vchordrq.",
        &EPSILON,
        0.0,
        4.0,
        GucContext::Userset,
        GucFlags::default(),
    );
}

pub fn probes() -> u32 {
    PROBES.get() as u32
}

pub fn epsilon() -> f32 {
    EPSILON.get() as f32
}
