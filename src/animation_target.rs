use std::time::Duration;

use crate::{easing::SINE_OUT, AnimationMode, AnimationTarget, Easing};

const DEFAULT_MODE: AnimationMode = AnimationMode::Start;
const DEFAULT_DURATION: Duration = Duration::new(0, 500 * 1000 * 1000);
const DEFAULT_EASING: Easing = SINE_OUT;

impl<T: Default> Default for AnimationTarget<T> {
    fn default() -> Self {
        AnimationTarget {
            target: T::default(),
            mode: DEFAULT_MODE,
            duration: DEFAULT_DURATION,
            easing: DEFAULT_EASING
        }
    }
}

impl<T> From<T> for AnimationTarget<T> {
    fn from(target: T) -> Self {
        AnimationTarget {
            target,
            mode: DEFAULT_MODE,
            duration: DEFAULT_DURATION,
            easing: DEFAULT_EASING,
        }
    }
}

// Allow users to call .into() on a tuple of any combination of duration, easing & mode

impl<T> From<(T, Duration)> for AnimationTarget<T> {
    fn from((target, duration): (T, Duration)) -> Self {
        AnimationTarget {
            target,
            mode: DEFAULT_MODE,
            duration,
            easing: DEFAULT_EASING,
        }
    }
}

impl<T> From<(T, Easing)> for AnimationTarget<T> {
    fn from((target, easing): (T, Easing)) -> Self {
        AnimationTarget {
            target,
            mode: DEFAULT_MODE,
            duration: DEFAULT_DURATION,
            easing,
        }
    }
}

impl<T> From<(T, AnimationMode)> for AnimationTarget<T> {
    fn from((target, mode): (T, AnimationMode)) -> Self {
        AnimationTarget {
            target,
            mode,
            duration: DEFAULT_DURATION,
            easing: DEFAULT_EASING,
        }
    }
}

impl<T> From<(T, Duration, Easing)> for AnimationTarget<T> {
    fn from((target, duration, easing): (T, Duration, Easing)) -> Self {
        AnimationTarget {
            target,
            mode: DEFAULT_MODE,
            duration,
            easing,
        }
    }
}

impl<T> From<(T, Easing, AnimationMode)> for AnimationTarget<T> {
    fn from((target, easing, mode): (T, Easing, AnimationMode)) -> Self {
        AnimationTarget {
            target,
            mode,
            duration: DEFAULT_DURATION,
            easing,
        }
    }
}

impl<T> From<(T, Duration, AnimationMode)> for AnimationTarget<T> {
    fn from((target, duration, mode): (T, Duration, AnimationMode)) -> Self {
        AnimationTarget {
            target,
            mode,
            duration,
            easing: DEFAULT_EASING,
        }
    }
}

impl<T> From<(T, Duration, Easing, AnimationMode)> for AnimationTarget<T> {
    fn from((target, duration, easing, mode): (T, Duration, Easing, AnimationMode)) -> Self {
        AnimationTarget {
            target,
            mode,
            duration,
            easing,
        }
    }
}
