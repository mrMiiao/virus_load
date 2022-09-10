#![no_main]

extern crate libc;
use libc::{srand, rand, time, puts, printf, system};

macro_rules! usleep {
    ($($x:tt)*) => {std::thread::sleep(std::time::Duration::from_micros(($($x)*) as u64));}
}

#[no_mangle]
unsafe extern fn main() {
    srand(time(&mut 0i64) as u32);
    puts("The loading is in progress. Please wait.\n\0".as_ptr() as *const i8);
    for i in 0..51 {
        for j in 0..50 {
            if j > i {
                printf("%c\0".as_ptr() as *const i8, 176u32);
            } else {
                printf("%c\0".as_ptr() as *const i8, 178u32);
            }
        }

        printf("%d%%\r\0".as_ptr() as *const i8, i * 2);

        if rand() % 2 != 0 {
            usleep!(100000 + rand() % 1000000);
        } else {
            usleep!(1 + rand() % 3);
        }
    }

    puts("\n\nThe virus has been installed!\n\n\0".as_ptr() as *const i8);
    system("pause\0".as_ptr() as *const i8);
}