// 표준 라이브러리에서 io, cmp 스코프 가져오기.
use std::cmp::Ordering;
use std::io;
// 외부 라이브러리인 rand에서 Rng(난수 생성기) 트레이트를 가져온다.
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // thread_rng() 메소드는 os에서 seed를 설정하고 해당 스레드에서 난수를 생성하는 메소드.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 이전 guess를 덮어 쓰는 shadowing이 가능하다.
        // trim() 메서드는 앞 뒤 개행문자, 캐리지 리턴 등을 지워주는 역할을 한다.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
