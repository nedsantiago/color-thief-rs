mod data_models;


fn main() {
    let color_box = data_models::Box {
        r_min: 10, r_max: 110,
        g_min: 10, g_max: 110,
        b_min: 10, b_max: 110,
    };
    println!("{:?}", color_box.volume());
}
