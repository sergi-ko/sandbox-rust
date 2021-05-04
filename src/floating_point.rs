pub fn test_floating_precision() {
    println!("{}", 0.3 / 0.0001); //2999.9999999995
    println!("{}", 0.3 * 10000_f64); //3000

    println!("{}", 0.3 / 0.05); //5.99999999999995
    println!("{}", 0.3 * 20_f64); //6
    println!("{}", 0.3 * 100_f64 / 5_f64); //6
}
