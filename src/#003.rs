fn main() {
    // 정수 뒤에 _u8, u_16등으로 타입을 명시할 수 있음
    let my_number = 9_u8;
    let other_number = 1_000_000_000u64;

    // 기본으로 소수점을 입력하면 f64(float) 타입으로 변수 선언
    let my_fnumber = 9.2;
    // 마찬가지로 다른 변수들도 타입이 자동 선언됨
    let my_bool = true;

    println!("my_number: {}, other number: {}", my_number, other_number);
    println!("my_fnumber: {}", my_fnumber);
    println!("my_bool: {}", my_bool);

    // Function 호출
    println!("Function: {}", give_number());

    // println!의 응용
    let my_city = &String::from("Seoul");
    let year = 2002;
    let population = 9_987_987;
    println!("{my_city} was founded in {year} with a population of {population}");
}

// -> i32 반환 데이터 타입
fn give_number() -> i32 {
    42 // return 생략 가능
}