use rand::Rng;

fn array_initial (arr: &mut[i32]) -> &mut [i32]{
    let mut i = 0;
    while i < arr.len() {
        arr[i] = rand::thread_rng().gen_range(1, 101);
        i = i + 1;
    }
    arr
}

fn print_array (arr: &mut[i32]){
    for item in arr {
        print!("{} ", item);
    }
    println!();
}

fn swap (arr: &mut [i32], i:usize, j:usize) -> &mut [i32]{
    let tmp: i32 = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
    arr
}

// interchange sort
fn interchange_sort(mut arr: &mut[i32]) -> &mut [i32]{
    let mut i = 0;
    let n = arr.len();
    while i < n-1 {
        let mut j = i + 1;
        while j <= n - 1 {
            if arr[i] > arr[j] {
                swap(&mut arr, i, j);
                print_array(arr);
            }
            
            j = j + 1;
        }
        i = i + 1;
    }

    arr
}

// sắp xếp nổi bọt
fn bubble_sort(mut arr: &mut[i32]) -> &mut[i32]{
    let mut i = arr.len() - 1;
    while i > 0 {
        let mut j = 0;
        while j < i {
            if arr[i] > arr[j] {
                swap(&mut arr, i, j);
                print_array(arr);
            }
            j = j + 1;
        } 
        i = i - 1;
    }
    arr
}

// sắp xếp chọn
fn selection_sort(mut arr: &mut[i32]) -> &mut[i32]{
    let n = arr.len();

    let mut i = 0;
    while i < n - 1 {
        let mut i_min = i;

        let mut j = i + 1;
        while j < n {
            if arr[j] < arr[i_min]{
                i_min = j;
            }
            j = j + 1;
        }

        if i_min != i {
            swap(&mut arr, i, i_min);
            print_array(arr);
        }
        i = i + 1;
    }
    arr
}

// sắp xếp chèn
fn insert_sort(mut arr: &mut[i32]) -> &mut [i32]{
    let n = arr.len();
    
    let mut j;
    let mut key:i32;

    let mut i = 1;
    while i < n {
        key = arr[i];
        j = i;
        while j >= 1 && arr[j - 1] > key {
            arr[j] = arr[j-1];
            j = j - 1;
        }
        arr[j] = key;
        print_array(arr);

        i = i + 1;
    }
    arr
}

// sắp xếp nhanh
fn quick_sort(mut arr: &mut[i32], lo: usize, hi: usize) -> &mut [i32]{
    if lo >= hi {
        print_array(arr);
        return arr;
    }

    let mid = lo + (hi - lo)/2;
    let mut i = lo;
    let mut j = hi;
    let key = arr[mid];

    while i<j {
        while arr[i] < key {
            i = i + 1;
        }
        while key < arr[j] {
            j = j - 1;
        }
        if i <= j {
            swap(arr, i, j);
            i = i + 1;
            j = j - 1;
        }
    }
    if lo < j {
        quick_sort(arr, lo, j);
    }
    if i < hi {
        quick_sort(arr, i, hi);
    }
    arr
}


fn main() {
    const N:i8 = 10;
    let mut arr: &mut [i32] = &mut [0; N as usize];
    array_initial(&mut arr);

    print_array(arr);
    println!("-----------------");
    quick_sort(arr, 0, arr.len() - 1);

    print_array(arr);
}
