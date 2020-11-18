use std::io;
fn main() {
    let mut temp = String::new();
    println!("温度を入力してください");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let temp: f32 = temp.trim().parse().expect("Failed to conversion");
    println!("摂氏{}[℃ ]は,華氏{}[℉ ]です.",temp,temp*1.8+32.0);
}
