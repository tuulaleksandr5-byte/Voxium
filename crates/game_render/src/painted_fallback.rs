//! Original smooth temporary artwork. This is NOT the user's missing PNG.
//! All patterns use continuous periodic coordinates, never per-pixel random noise.
use std::f32::consts::TAU;

pub fn atlas() -> Vec<u8> {
    let mut out = Vec::with_capacity(768 * 384 * 4);
    for y in 0..384 {
        for x in 0..768 {
            out.extend(pixel(
                (y / 128 * 6 + x / 128) as u8,
                (x % 128) as f32 / 128.,
                (y % 128) as f32 / 128.,
            ));
        }
    }
    out
}
fn pixel(layer: u8, x: f32, y: f32) -> [u8; 4] {
    let wave = |n: f32, m: f32| (TAU * (x * n + y * m)).sin();
    let smooth = wave(1., 2.) * 3. + wave(3., -1.) * 2.;
    let (base, value, alpha) = match layer {
        0 => ([84., 139., 40.], leaves(x, y, 8.) * 30. + smooth, 255.),
        1 if y < 0.22 + wave(3., 0.) * 0.025 => ([84., 139., 40.], leaves(x, y, 8.) * 30., 255.),
        1 | 2 => ([126., 85., 53.], stones(x, y, 6.).0 * 12. + smooth, 255.),
        3 | 4 | 6 | 11 => {
            let scale = if layer == 6 || layer == 11 { 5. } else { 2. };
            let (cell, edge) = stones(x, y, scale);
            let crack = -24. * (1. - smoothstep(0.018, 0.05, edge));
            let base = match layer {
                4 => [48., 49., 67.],
                11 => [100., 113., 80.],
                _ => [126., 131., 139.],
            };
            (
                base,
                cell * 13. + edge.min(0.12) * 48. + crack + smooth,
                255.,
            )
        }
        5 | 16 => (
            [224., 203., 153.],
            wave(1., 4.) * 5. + wave(2., 1.) * 3.,
            255.,
        ),
        7 => (
            [127., 84., 42.],
            (TAU * (x * 7. + wave(1., 1.) * 0.08)).sin() * 17. + smooth,
            255.,
        ),
        8 => {
            let dx = x - 0.5;
            let dy = y - 0.5;
            let r = (dx * dx + dy * dy).sqrt();
            (
                [176., 133., 72.],
                (r * 65.).sin() * 12. - smoothstep(0.40, 0.48, r) * 32.,
                255.,
            )
        }
        9 => ([65., 124., 48.], leaves(x, y, 6.) * 34. + smooth, 255.),
        10 => {
            let seam = 1. - smoothstep(0.008, 0.025, (y * 4.).fract().min(1. - (y * 4.).fract()));
            (
                [172., 122., 61.],
                wave(1., 13.) * 4. - seam * 33. + smooth,
                255.,
            )
        }
        12 => {
            let border = x.min(1. - x).min(y.min(1. - y));
            (
                [164., 219., 230.],
                (1. - smoothstep(0.01, 0.045, border)) * 35. + wave(1., 1.) * 3.,
                255.,
            )
        }
        13 => (
            [48., 157., 203.],
            wave(2., 3.) * 12. + wave(3., -2.) * 10. + wave(1., 1.) * 6.,
            255.,
        ),
        14 => (
            [218., 233., 247.],
            wave(2., 3.) * 7. + wave(3., -2.) * 4.,
            255.,
        ),
        15 => (
            [211., 184., 128.],
            (TAU * (y * 5. + wave(1., 0.) * 0.09)).sin() * 9.,
            255.,
        ),
        _ => ([0.; 3], 0., 0.),
    };
    let c = base.map(|v| (v + value).clamp(0., 255.) as u8);
    [c[0], c[1], c[2], alpha as u8]
}
fn smoothstep(a: f32, b: f32, v: f32) -> f32 {
    let t = ((v - a) / (b - a)).clamp(0., 1.);
    t * t * (3. - 2. * t)
}
fn leaves(x: f32, y: f32, scale: f32) -> f32 {
    let px = (x * scale + (y * scale).floor() * 0.5).fract() - 0.5;
    let py = (y * scale).fract() - 0.5;
    let d = ((px + py * 0.4).powi(2) * 4. + py * py * 2.).sqrt();
    (1. - smoothstep(0.1, 0.75, d)) * (1. + px * 0.5) - 0.5
}
fn stones(x: f32, y: f32, scale: f32) -> (f32, f32) {
    let gx = (x * scale).floor() as i32;
    let gy = (y * scale).floor() as i32;
    let mut first = f32::MAX;
    let mut second = f32::MAX;
    let mut value = 0.;
    for oy in -1..=1 {
        for ox in -1..=1 {
            let cx = gx + ox;
            let cy = gy + oy;
            let sx = cx.rem_euclid(scale as i32) as f32;
            let sy = cy.rem_euclid(scale as i32) as f32;
            let h = (sx * 12.9898 + sy * 78.233).sin();
            let dx = x * scale - (cx as f32 + 0.5 + h * 0.22);
            let dy = y * scale - (cy as f32 + 0.5 + (h * 41.).sin() * 0.22);
            let d = (dx * dx + dy * dy).sqrt();
            if d < first {
                second = first;
                first = d;
                value = h;
            } else if d < second {
                second = d;
            }
        }
    }
    (value, second - first)
}
