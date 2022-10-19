/// Primitive Types
/// plus sign
/// minus sign
/// i8, i16, i32, i64, i128, isize
/// isize -> 32비트 -> i32 , isize -> 64비트 -> i64
/// u8, u16, u32, u64, u128, usize
fn main() {
    let my_number = 100; // 타입 선언 안할시 기본 i32으로 선언됨
    let my_order_number: u8 = 50; // 타입 선언
    let thrid_number = my_number + my_order_number; // 다른 타입끼리의 연산은 불가능
    println!("{} + {} = {}", my_number, my_order_number, thrid_number);

    let my_number: u8 = 100;
    let my_number2: u32 = 200;
    println!("{} + {} = {}", my_number, my_number2, my_number + my_number2 as u8); // as를 통해 타입을 변환할 수 있음
    //char Type은 4바이트
    let my_char: char = 'a';
    println!("my_char: {}", my_char);

    //bool Type
    let my_bool: bool = true;
    println!("my_bool: {}", my_bool);

    //Tuple Type
    let my_tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("my_tuple: {:?}", my_tuple);

    let name: &str = "Rust"; // str 타입은 문자열 슬라이스
    println!("Hello, {}!", name);
}