use std::io;
use rand::Rng;

// Khởi tạo mảng
pub fn array_initial (arr: &mut [i32]) -> &mut [i32]{
    let mut i: usize = 0;
    let n = arr.len();
    while i < n {
        arr[i] = rand::thread_rng().gen_range(1, 101);
        i = i + 1;
    }

    arr
}

// Khởi tạo mảng tăng dần
pub fn array_initial_ordered (arr: &mut [i32]) -> &mut [i32] {
    let mut i: usize = 1;
    let n = arr.len();
    while i < n {
        arr[i] = arr[i-1] + rand::thread_rng().gen_range(1, 101);
        i = i + 1;
    }

    arr
}


// tìm kiếm tuyến tính
fn linear_search(arr: &mut [i32], x: i32) -> i32 {
    let length = arr.len() - 1; // mảng bắt đầu từ 0
    arr[length] = x;
    
    let mut i = 0;

    loop {
        if x == arr[i] {
            return i as i32;
        }
        i = i + 1;
    }
}

// Tìm kiếm nhị phân
// @param mảng đã được sắp xếp, số cần tìm
fn binary_search(arr: &mut [i32], x: i32) -> i8 {
    let mut lo: i8 = 0;
    let mut hi: i8 = (arr.len() - 1) as i8;
    let mut mid: i8 = 0;

    while lo <= hi {
        mid = lo + (hi - lo)/2;
        let val = arr[mid as usize];

        if x == val {
            return mid;
        }

        if x < val {
            hi = mid - 1; 
        }

        if x > val {
            lo = mid + 1;
        }
    }

    -1
}

// Tìm kiếm nội suy
fn interpolation_search(arr: &mut [i32], x:i32) -> i32 {
    let mut lo: i32 = 0;
    let mut hi: i32 = (arr.len() - 1) as i32;
    
    while lo <= hi {
        let mid = lo + ((x - arr[lo as usize]) * (hi - lo) / (arr[hi as usize] - arr[lo as usize]));
        let val = arr[mid as usize];

        if x == val {
            return mid;
        }

        if x < val {
            hi = mid - 1; 
        }

        if x > val {
            lo = mid + 1;
        }
    }

    -1
}

fn main(){
    const N:usize = 10;
    let mut arr:&mut [i32] = &mut [0 ; N+1]; // Khởi tạo mảng mới n + 1 phần tử 0 

    
    //array_initial(&mut arr); // không có thứ tự
    array_initial_ordered(&mut arr);

    println!("Xuất mảng");
    for item in arr.iter() {
        print!(" {} ", item);
    }
    println!();
    

    println!("Nhập một số cần tìm");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Không nhập được");
    let x: i32 = match x.trim().parse(){
        Ok(num) => num,
        Err(_) => 0, // nếu nhập sai trả về số 0
    };

    // Tìm kiếm tuần tự
    // let pos = linear_search(arr, x);

    // if pos == (N) as i32 {
    //     print!("Không tìm thấy");
    // }else{
    //     print!("Tìm thấy {} tại vị trí {}", x, pos);
    // }

    // tìm kiếm bằng tìm kiếm nhị phân
    let pos = interpolation_search(arr, x);

    if pos == -1 {
        print!("Không tìm thấy");
    }else{
        print!("Tìm thấy {} tại vị trí {}", x, pos);
    }

}