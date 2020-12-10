use std::io;
use rand::Rng; // Rng xác định phương pháp tạo số ngẫu nhiên
use std::cmp::Ordering; // cmp là compare: so sánh đó

fn main() {
    println!("Đoán số");
    println!("Nhập 1 số để đoán");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // mut là mutable để có thể thay đổi giá trị cho biến
        // biến trong Rust mặc định không thay đổi giá trị
        let mut guess = String::new(); //Đây là kiểu chuổi không so sánh với kiểu số được

        io::stdin().read_line(&mut guess).expect("Nhập không thành công"); // đọc dữ liệu nhập vào

        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num, // Kiểu số
            Err(_) => continue, // _ : tất cả các kiểu
        }; // ép kiểu
        // Rust cho phép đặt tên biến giống với một tên biến khác kiểu
        // Dùng để ép kiểu
        // Gọi là shadowing

        println!("số vừa nhập là: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Nhỏ quá"),
            Ordering::Greater => println!("Lớn quá"),
            Ordering::Equal => {
                println!("Đúng rồi");
                break; // dùng chung với lệnh lặp
            },
        }
    }
    
    
}
