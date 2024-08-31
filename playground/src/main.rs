use num::complex::Complex;

fn main() {
    numeric_types();

    println!("\n\n ================================ \n\n");

    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}

fn numeric_types() {
    let a = 30_i32;

    assert!(a > 10);

    let forty_twos = [42.0, 42f32, 42.0f32];

    println!("{:04.1}", forty_twos[0]);

    let three = 0b11;
    let thirty = 0o32;
    let three_hundred = 0x12C;
    let forty_two = 42;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
    println!("base 2: {:b} {:o} {:x}", forty_two, forty_two, forty_two);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc: (f32)");
    println!("      0.1 + 0.2 = {:x}", (abc.0 + abc.1).to_bits());
    println!("            0.3 = {:x}", (abc.2).to_bits());

    println!("abc: (f64)");
    println!("      0.1 + 0.2 = {:x}", (xyz.0 + xyz.1).to_bits());
    println!("            0.3 = {:x}", (xyz.2).to_bits());

    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let abs_diff = result - desired;

    assert!(abs_diff < f32::EPSILON);
}
