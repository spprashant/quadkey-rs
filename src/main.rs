mod tile_system;

fn main() {
    let g = tile_system::Geo{lat: 70.0, lon: -120.0};
    let pixel = tile_system::geo_to_pixel(g, 17);
    println!("{:?}", pixel);
}
