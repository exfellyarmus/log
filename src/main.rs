use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use chrono::Local; // 시간을 다루기 위한 모듈

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("사용법: cargo run -- [add|list] [내용]");
        return;
    }

    let command = &args[1];
    let file_path = "logs.txt";

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("추가할 내용을 입력해주세요.");
                return;
            }
            let content = &args[2];
            
            // 현재 지역 시간을 가져와서 [yyyy-mm-dd hh:mm] 형식으로 변환합니다.
            let now = Local::now();
            let timestamp = now.format("%Y-%m-%d %H:%M").to_string();

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
                .expect("파일을 열 수 없습니다.");

            // "[날짜 시간] 내용" 형식으로 파일에 씁니다.
            writeln!(file, "[{}] {}", timestamp, content).expect("쓰기 실패");
            println!("저장됨: [{}] {}", timestamp, content);
        }
        "list" => {
            match read_to_string(file_path) {
                Ok(contents) => {
                    println!("--- 저장된 로그 목록 ---");
                    print!("{}", contents);
                }
                Err(_) => println!("저장된 로그가 없습니다."),
            }
        }
        _ => println!("알 수 없는 명령어: {}", command),
    }
}