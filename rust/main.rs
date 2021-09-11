fn main() {
    let s = String::from("無職転生はいいぞ");
    println!("{}", s);
}// ここでsがスコープから抜けるので"無職転生はいいぞ"という値も破棄される
