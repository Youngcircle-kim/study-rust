use std::io::{self};
fn convert_c_to_f(temperature: f32) -> f32 {
    (temperature - 32.0) * 1.8
}
fn convert_f_to_c(temperature: f32) -> f32 {
    (temperature) * 1.8 + 32.0
}
// 화씨, 섭씨 변환 프로그램
fn main() {
    println!("화씨, 섭씨 상호 변환 프로그램");

    let mut system: String = String::new();
    let mut input = String::new();

    println!(
        "아래 보기에 따라 변경할 온도의 숫자를 입력해 주세요.\n 1. 화씨 -> 섭씨 2. 섭씨 -> 화씨"
    );

    io::stdin()
        .read_line(&mut system)
        .expect("Plz input the correct answer.");

    println!("온도를 입력해주세요.");
    io::stdin()
        .read_line(&mut input)
        .expect("Plz input the correct answer.");

    let system = system.trim();
    let temperature: f32 = input.trim().parse().expect("Plz input the valid number");

    if system == "1" {
        let result: f32 = convert_f_to_c(temperature);
        println!("화씨에서 섭씨 변환 결과: {:.2} °C", result);
    } else if system == "2" {
        let result: f32 = convert_c_to_f(temperature);
        println!("{result}");
    } else {
        println!("Invalid Input Number");
    }
}
