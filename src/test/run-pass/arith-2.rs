fn main() -> () {
    let int i32_c = 0x10101010;
    check (i32_c + i32_c * 2 / 3 * 2 + (i32_c - 7 % 3) == 
           i32_c + (((i32_c * 2) / 3) * 2) + (i32_c - (7 % 3)));
}