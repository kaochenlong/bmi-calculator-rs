use std::io;
mod bmi;

fn main() {
    let height = ask_question("請輸入您的身高（單位：公分）：");
    let weight = ask_question("接著請輸入您的體重（單位：公斤）：");

    let bmi = bmi::calculator(height, weight);
    let message = if bmi >= 0.0 && bmi <= 18.5 {
        "「體重過輕」，需要多運動，均衡飲食，以增加體能，維持健康！"
    } else if bmi > 18.5 && bmi <= 24.0 {
        "恭喜！「健康體重」，要繼續保持！"
    } else if bmi > 24.0 && bmi <= 27.0 {
        "「體重過重」了，要小心囉，趕快力行「健康體重管理」！"
    } else if bmi > 27.0 {
        "啊～「肥胖」，需要立刻力行「健康體重管理」囉！"
    } else {
        "無法計算"
    };

    println!("您的 BMI 是 {}，{}", bmi, message);
}

fn ask_question(question: &str) -> f64 {
    println!("{}", question);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("無法讀取輸入");

    input.trim().parse().expect("無法轉換")
}
