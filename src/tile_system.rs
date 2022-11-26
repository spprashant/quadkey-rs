use std::cmp::{min, max};
use std::f64::consts::{PI};

#[derive(Debug)]
pub struct Geo {
    pub lon: f64,
    pub lat: f64
}

const EARTH_RADIUS:i32 = 6378137;
const LATITUDE_RANGE: (f64, f64) = (-85.05112878, 85.05112878);
const LONGITUDE_RANGE: (f64, f64) = (-180.0, 180.0);

fn clip(n: f64, min_max: (f64, f64)) -> f64 {
    let (x, y) = min_max;
    return y.min(n.max(x));
}

fn map_size(level: u16) -> u32 {
    return 256u32 << level
}

pub fn geo_to_pixel(geo: Geo, level: u16) -> (f64, f64) {
    let lat = clip(geo.lat, LATITUDE_RANGE);
    let lon = clip(geo.lon, LONGITUDE_RANGE);

    let x: f64 = (lon + 180.0) / 360.0;
    let sin_lat = (f64::from(lat) * PI / 180.0f64).sin();
    let y: f64 = 0.5 - ((1.0f64 + sin_lat) / (1.0f64 - sin_lat)).log10() / (4.0f64 * PI);

    let map_size = f64::from(map_size(level));

    let pixel_x = clip(x * map_size + 0.5f64, (0.0f64, map_size - 1.0f64));
    let pixel_y = clip(y * map_size + 0.5f64, (0.0f64, map_size - 1.0f64)); 

    return (pixel_x, pixel_y)
}

