use std::cmp::{min, max};
use std::f64::consts::{PI};

#[derive(Debug)]
pub struct Geo {
    pub lon: f64,
    pub lat: f64
}

impl PartialEq for Geo {
    fn eq(&self, other: &Self) -> bool {
        self.lat == other.lat && self.lon == other.lon
    }
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

pub fn geo_to_pixel(geo: Geo, level: u16) -> (i64, i64) {
    let lat = clip(geo.lat, LATITUDE_RANGE);
    let lon = clip(geo.lon, LONGITUDE_RANGE);

    let x: f64 = (lon + 180.0) / 360.0;
    let sin_lat = (f64::from(lat) * PI / 180.0f64).sin();
    let y: f64 = 0.5 - ((1.0f64 + sin_lat) / (1.0f64 - sin_lat)).ln() / (4.0f64 * PI);

    let map_size = f64::from(map_size(level));

    let pixel_x = clip(x * map_size + 0.5f64, (0.0f64, map_size - 1.0f64));
    let pixel_y = clip(y * map_size + 0.5f64, (0.0f64, map_size - 1.0f64)); 

    return (pixel_x as i64, pixel_y as i64)
}

pub fn pixel_to_geo(pixel: (i64, i64), level: u16) -> Geo {
    let pixel_x: f64 = pixel.0 as f64;
    let pixel_y: f64 = pixel.1 as f64;

    let map_size: f64 = map_size(level) as f64;

    let x: f64 = (clip(pixel_x, (0.0, map_size - 1f64)) / map_size) - 0.5f64;
    let y: f64 = 0.5f64 - (clip(pixel_y, (0.0, map_size - 1f64)) / map_size);
    let lat: f64 = 90.0f64 - (360.0f64 * f64::exp(-y * 2f64 * PI).atan() / PI);
    let lon: f64 = 360.0f64 * x;
    let rounded_lat = f64::round(lat * 100f64) / 100f64;
    let rounded_lon = f64::round(lon * 100f64) / 100f64;
    return Geo{lat : rounded_lat, lon: rounded_lon}
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo_to_pixel() {
        let geo: Geo = Geo{lat : 40.0, lon : -105.0};
        let level = 7;
        let result = (6827, 12405);
        assert_eq!(result, geo_to_pixel(geo, level));
    }

    #[test]
    fn test_pixel_to_geo() {
        let pixel = (6827, 12405);
        let level = 7;
        let result: Geo = Geo{lat : 40.0, lon : -105.0};
        assert_eq!(result, pixel_to_geo(pixel, level));
    }
}
