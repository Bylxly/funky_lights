pub(crate) struct Normalizer {
    max_sub_bass: f32,
    max_bass: f32,
    max_mid: f32,
    max_high: f32,
    smoothed_sub_bass: f32,
    smoothed_bass: f32,
    smoothed_mid: f32,
    smoothed_high: f32,
    smoothing: f32,
}

impl Normalizer {
    pub(crate) fn new(smoothing: f32) -> Self {

        Self {
            max_sub_bass: 0f32,
            max_bass: 0f32,
            max_mid: 0f32,
            max_high: 0f32,
            smoothed_sub_bass: 0f32,
            smoothed_bass: 0f32,
            smoothed_mid: 0f32,
            smoothed_high: 0f32,
            smoothing
        }
    }


    pub(crate) fn process(&mut self, sub_bass: f32, bass: f32, mid: f32, high: f32) -> (f32, f32, f32, f32) {
        // update maximum
        if sub_bass > self.max_sub_bass {self.max_sub_bass = sub_bass}
        if bass > self.max_bass {self.max_bass = bass}
        if mid > self.max_mid {self.max_mid = mid}
        if high > self.max_high {self.max_high = high}

        // decrease maximum slowly
        self.max_sub_bass *= 0.999;
        self.max_bass *= 0.999;
        self.max_mid *= 0.999;
        self.max_high *= 0.999;

        // normalize
        let normalized_sub_bass = if self.max_sub_bass > 0.0 {sub_bass / self.max_sub_bass} else {0.0};
        let normalized_bass = if self.max_bass > 0.0 {bass / self.max_bass} else {0.0};
        let normalized_mid = if self.max_mid > 0.0 {mid / self.max_mid} else {0.0};
        let normalized_high = if self.max_high > 0.0 {high / self.max_high} else {0.0};

        // apply smoothing
        self.smoothed_sub_bass = self.smoothed_sub_bass * self.smoothing
            + normalized_sub_bass * (1.0 - self.smoothing);
        self.smoothed_bass = self.smoothed_bass * self.smoothing
            + normalized_bass * (1.0 - self.smoothing);
        self.smoothed_mid = self.smoothed_mid * self.smoothing
            + normalized_mid * (1.0 - self.smoothing);
        self.smoothed_high = self.smoothed_high * self.smoothing
            + normalized_high * (1.0 - self.smoothing);

        (self.smoothed_sub_bass, self.smoothed_bass, self.smoothed_mid, self.smoothed_high)
    }

}