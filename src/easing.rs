use ::std::f64::consts::PI;

const C1: f64 = 1.70158;
const C2: f64 = C1 * 1.525;
const C3: f64 = C1 + 1.0;
const C4: f64 = (2.0 * PI) / 3.0;
const C5: f64 = (2.0 * PI) / 4.5;

/// <https://easings.net/#easeInBack>
pub fn back_in(t: f64) -> f64 {
    C3 * t * t * t - C1 * t * t
}

/// <https://easings.net/#easeOutBack>
pub fn back_out(t: f64) -> f64 {
    1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
}

/// <https://easings.net/#easeInOutBack>
pub fn back_in_out(t: f64) -> f64 {
    if t < 0.5 {
        ((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
    } else {
        ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
    }
}

/// <https://easings.net/#easeInBounce>
pub fn bounce_in(t: f64) -> f64 {
    1.0 - bounce_out(1.0 - t)
}

/// <https://easings.net/#easeOutBounce>
pub fn bounce_out(t: f64) -> f64 {
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
}

/// <https://easings.net/#easeInOutBounce>
pub fn bounce_in_out(t: f64) -> f64 {
    if t < 0.5 {
        (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
    } else {
        (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
    }
}

/// <https://easings.net/#easeInCirc>
pub fn circ_in(t: f64) -> f64 {
    1.0 - (1.0 - t.powi(2)).sqrt()
}

/// <https://easings.net/#easeOutCirc>
pub fn circ_out(t: f64) -> f64 {
    (1.0 - (t - 1.0).powi(2)).sqrt()
}

/// <https://easings.net/#easeInOutCirc>
pub fn circ_in_out(t: f64) -> f64 {
    if t < 0.5 {
        (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}

/// <https://easings.net/#easeInCubic>
pub fn cubic_in(t: f64) -> f64 {
    t * t * t
}

/// <https://easings.net/#easeOutCubic>
pub fn cubic_out(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

/// <https://easings.net/#easeInOutCubic>
pub fn cubic_in_out(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// <https://easings.net/#easeInElastic>
pub fn elastic_in(t: f64) -> f64 {
    if t <= 0.0 {
        0.0
    } else if 1.0 <= t {
        1.0
    } else {
        -2f64.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * C4).sin()
    }
}

/// <https://easings.net/#easeOutElastic>
pub fn elastic_out(t: f64) -> f64 {
    if t <= 0.0 {
        0.0
    } else if 1.0 <= t {
        1.0
    } else {
        2f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
    }
}

/// <https://easings.net/#easeInOutElastic>
pub fn elastic_in_out(t: f64) -> f64 {
    if t <= 0.0 {
        0.0
    } else if 1.0 <= t {
        1.0
    } else if t < 0.5 {
        -(2f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0
    } else {
        (2f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0 + 1.0
    }
}

/// <https://easings.net/#easeInExpo>
pub fn expo_in(t: f64) -> f64 {
    if t <= 0.0 {
        0.0
    } else {
        2f64.powf(10.0 * t - 10.0)
    }
}

/// <https://easings.net/#easeOutExpo>
pub fn expo_out(t: f64) -> f64 {
    if 1.0 <= t {
        1.0
    } else {
        1.0 - 2f64.powf(-10.0 * t)
    }
}

/// <https://easings.net/#easeInOutExpo>
pub fn expo_in_out(t: f64) -> f64 {
    if t <= 0.0 {
        0.0
    } else if 1.0 <= t {
        1.0
    } else if t < 0.5 {
        2f64.powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - 2f64.powf(-20.0 * t + 10.0)) / 2.0
    }
}

pub fn linear(t: f64) -> f64 {
    t
}

/// A linear easing that goes from `1.0` to `0.0`.
pub fn reverse(t: f64) -> f64 {
    1.0 - t
}

/// <https://easings.net/#easeInQuad>
pub fn quad_in(t: f64) -> f64 {
    t * t
}

/// <https://easings.net/#easeOutQuad>
pub fn quad_out(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(2)
}

/// <https://easings.net/#easeInOutQuad>
pub fn quad_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

/// <https://easings.net/#easeInQuart>
pub fn quart_in(t: f64) -> f64 {
    t * t * t * t
}

/// <https://easings.net/#easeOutQuart>
pub fn quart_out(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(4)
}

/// <https://easings.net/#easeInOutQuart>
pub fn quart_in_out(t: f64) -> f64 {
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
    }
}

/// <https://easings.net/#easeInQuint>
pub fn quint_in(t: f64) -> f64 {
    t * t * t * t * t
}

/// <https://easings.net/#easeOutQuint>
pub fn quint_out(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(5)
}

/// <https://easings.net/#easeInOutQuint>
pub fn quint_in_out(t: f64) -> f64 {
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
    }
}

/// <https://easings.net/#easeInSine>
pub fn sine_in(t: f64) -> f64 {
    1.0 - (t * PI / 2.0).cos()
}

/// <https://easings.net/#easeOutSine>
pub fn sine_out(t: f64) -> f64 {
    (t * PI / 2.0).sin()
}

/// <https://easings.net/#easeInOutSine>
pub fn sine_in_out(t: f64) -> f64 {
    -((PI * t).cos() - 1.0) / 2.0
}
