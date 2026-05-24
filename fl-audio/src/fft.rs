use fl_config::AudioConfig;
use num_complex::Complex;
use rustfft::FftPlanner;

pub(crate) fn analyze(samples: &[f32], config: &AudioConfig) -> (f32, f32, f32, f32) {
    // apply Hann-Window
    let n = samples.len();
    let mut windowed: Vec<Complex<f32>> = samples
        .iter()
        .enumerate()
        .map(|(i, &sample)| {
            let hann = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (n - 1) as f32).cos());
            Complex { re: sample * hann, im: 0.0}
        })
        .collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(&mut windowed);

    let magnitudes: Vec<f32> = windowed[..n/2].iter().map(|b| b.norm()).collect();

    let sub_bass = band_average(&magnitudes,
                                config.get_sub_bass().get_min(),
                                config.get_sub_bass().get_max(),
                                config.get_sample_rate(),
                                config.get_buffer_size());

    let bass = band_average(&magnitudes,
                                config.get_bass().get_min(),
                                config.get_bass().get_max(),
                                config.get_sample_rate(),
                                config.get_buffer_size());

    let mid = band_average(&magnitudes,
                                config.get_mid().get_min(),
                                config.get_mid().get_max(),
                                config.get_sample_rate(),
                                config.get_buffer_size());

    let high = band_average(&magnitudes,
                                config.get_high().get_min(),
                                config.get_high().get_max(),
                                config.get_sample_rate(),
                                config.get_buffer_size());

    (sub_bass, bass, mid, high)
}

fn band_average(
    magnitudes: &[f32],
    min_hz: f32,
    max_hz: f32,
    sample_rate: u32,
    buffer_size: u32) -> f32 {
    let mut sum = 0.0;
    let mut count = 0;
    for (i, &magnitude) in magnitudes.iter().enumerate() {
        let bin_freq = i as f32 * sample_rate as f32 / buffer_size as f32;
        if bin_freq >= min_hz && bin_freq <= max_hz {
            sum += magnitude;
            count += 1
        }
    }
    if count == 0 {0.0} else {sum / count as f32}
}