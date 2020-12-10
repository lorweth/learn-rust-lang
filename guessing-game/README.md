# Làm guessing game với Rust

Khai báo biến `let` với `mut` để có thể thay đổi giá trị của biến. Mặc định biến của Rust là bất biến

Rust cho phép đặt tên biến trùng với 1 biến khác nếu khác kiểu dữ liệu, dùng cho việc ép kiểu gọi là shadowing.

Ép kiểu bằng cách 
```Rust
let mut guess = String::new();

// Ép kiểu String sang kiểu Uint32
// Bắt lỗi với expect
let guess: u32 = guess.trim().parse().expect("Please type a number!"); 

// sử dụng match để bắt lỗi
let guess: i32 = match guess.trim().parse(){
    Ok(num) => num, // Kiểu số
    Err(_) => continue, // _ : tất cả các kiểu
};
```

Dùng thư viện `std::cmp::Ordering;` để so sánh 2 giá trị

```Rust
match guess.cmp(&secret_number){
    Ordering::Less => println!("Nhỏ quá"),
    Ordering::Greater => println!("Lớn quá"),
    Ordering::Equal => {
        println!("Đúng rồi");
        break; // dùng chung với lệnh lặp
    },
}
```