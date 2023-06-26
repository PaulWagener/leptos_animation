use ::std::f64::consts::PI;
use crate::Easing;

const C1: f64 = 1.70158;
const C2: f64 = C1 * 1.525;
const C3: f64 = C1 + 1.0;
const C4: f64 = (2.0 * PI) / 3.0;
const C5: f64 = (2.0 * PI) / 4.5;

/// <https://easings.net/#easeInBack>
pub const BACK_IN: Easing = |t: f64| -> f64 {
    C3 * t * t * t - C1 * t * t
};

/// <https://easings.net/#easeOutBack>
pub const BACK_OUT: Easing = |t: f64| -> f64 {
    1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
};

/// <https://easings.net/#easeInOutBack>
pub const BACK_IN_OUT: Easing = |t: f64| -> f64 {
    if t < 0.5 {
        ((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
    } else {
        ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
    }
};

/// <https://easings.net/#easeInBounce>
pub const BOUNCE_IN: Easing = |t: f64| -> f64 {
    1.0 - BOUNCE_OUT(1.0 - t)
};

/// <https://easings.net/#easeOutBounce>
pub const BOUNCE_OUT: Easing = |t: f64| -> f64 {
    const N1: f64 = 7.5625;
    const D1: f64 = 2.75;
    if t < 1.0 / D1 {
        return N1 * t * t;
    } else if t < 2.0 / D1 {
        return N1 * (t - 1.5 / D1).powi(2) + 0.75;
    } else if t < 2.5 / D1 {
        return N1 * (t - 2.25 / D1).powi(2) + 0.9375;
    } else {
        return N1 * (t - 2.625 / D1).powi(2) + 0.984375;
    }
};

/// <https://easings.net/#easeInOutBounce>
pub const BOUNCE_IN_OUT: Easing = |t: f64| -> f64 {
    if t < 0.5 {
        (1.0 - BOUNCE_OUT(1.0 - 2.0 * t)) / 2.0
    } else {
        (1.0 + BOUNCE_OUT(2.0 * t - 1.0)) / 2.0
    }
};

/// <https://easings.net/#easeInCirc>
pub const CIRC_IN: Easing = |t: f64| -> f64 {
    1.0 - (1.0 - t.powi(2)).sqrt()
};

/// <https://easings.net/#easeOutCirc>
pub const CIRC_OUT: Easing = |t: f64| -> f64 {
    (1.0 - (t - 1.0).powi(2)).sqrt()
};

/// <https://easings.net/#easeInOutCirc>
pub const CIRC_IN_OUT: Easing = |t: f64| -> f64 {
    if t < 0.5 {
        (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
};

/// <https://easings.net/#easeInCubic>
pub const CUBIC_IN: Easing = |t: f64| -> f64 {
    t * t * t
};

/// <https://easings.net/#easeOutCubic>
pub const CUBIC_OUT: Easing = |t: f64| -> f64 {
    1.0 - (1.0 - t).powi(3)
};

/// <https://easings.net/#easeInOutCubic>
pub const CUBIC_IN_OUT: Easing = |t: f64| -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
};

/// <https://easings.net/#easeInElastic>
pub const ELASTIC_IN: Easing = |t: f64| -> f64 {
    if t <= 0.0 {
        0.0
    } else if 1.0 <= t {
        1.0
    } else {
        -2f64.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * C4).sin()
    }
};

/// <https://easings.net/#easeOutElastic>
pub const ELASTIC_OUT: Easing = |t: f64| -> f64 {
    if t <= 0.0 {
        0.0
    } else if 1.0 <= t {
        1.0
    } else {
        2f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
    }
};

/// <https://easings.net/#easeInOutElastic>
pub const ELASTIC_IN_OUT: Easing = |t: f64| -> f64 {
    if t <= 0.0 {
        0.0
    } else if 1.0 <= t {
        1.0
    } else if t < 0.5 {
        -(2f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0
    } else {
        (2f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0 + 1.0
    }
};

/// <https://easings.net/#easeInExpo>
pub const EXPO_IN: Easing = |t: f64| -> f64 {
    if t <= 0.0 {
        0.0
    } else {
        2f64.powf(10.0 * t - 10.0)
    }
};

/// <https://easings.net/#easeOutExpo>
pub const EXPO_OUT: Easing = |t: f64| -> f64 {
    if 1.0 <= t {
        1.0
    } else {
        1.0 - 2f64.powf(-10.0 * t)
    }
};

/// <https://easings.net/#easeInOutExpo>
pub const EXPO_IN_OUT: Easing = |t: f64| -> f64 {
    if t <= 0.0 {
        0.0
    } else if 1.0 <= t {
        1.0
    } else if t < 0.5 {
        2f64.powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - 2f64.powf(-20.0 * t + 10.0)) / 2.0
    }
};

pub const LINEAR: Easing = |t: f64| -> f64 {
    t
};

/// A linear easing that goes from `1.0` to `0.0`.
pub const REVERSE: Easing = |t: f64| -> f64 {
    1.0 - t
};

/// <https://easings.net/#easeInQuad>
pub const QUAD_IN: Easing = |t: f64| -> f64 {
    t * t
};

/// <https://easings.net/#easeOutQuad>
pub const QUAD_OUT: Easing = |t: f64| -> f64 {
    1.0 - (1.0 - t).powi(2)
};

/// <https://easings.net/#easeInOutQuad>
pub const QUAD_IN_OUT: Easing = |t: f64| -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
};

/// <https://easings.net/#easeInQuart>
pub const QUART_IN: Easing = |t: f64| -> f64 {
    t * t * t * t
};

/// <https://easings.net/#easeOutQuart>
pub const QUART_OUT: Easing = |t: f64| -> f64 {
    1.0 - (1.0 - t).powi(4)
};

/// <https://easings.net/#easeInOutQuart>
pub const QUART_IN_OUT: Easing = |t: f64| -> f64 {
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
    }
};

/// <https://easings.net/#easeInQuint>
pub const QUINT_IN: Easing = |t: f64| -> f64 {
    t * t * t * t * t
};

/// <https://easings.net/#easeOutQuint>
pub const QUINT_OUT: Easing = |t: f64| -> f64 {
    1.0 - (1.0 - t).powi(5)
};

/// <https://easings.net/#easeInOutQuint>
pub const QUINT_IN_OUT: Easing = |t: f64| -> f64 {
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
    }
};

/// <https://easings.net/#easeInSine>
pub const SINE_IN: Easing = |t: f64| -> f64 {
    1.0 - (t * PI / 2.0).cos()
};

/// <https://easings.net/#easeOutSine>
pub const SINE_OUT: Easing = |t: f64| -> f64 {
    (t * PI / 2.0).sin()
};

/// <https://easings.net/#easeInOutSine>
pub const SINE_IN_OUT: Easing = |t: f64| -> f64 {
    -((PI * t).cos() - 1.0) / 2.0
};
