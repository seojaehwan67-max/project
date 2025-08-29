use std::env;
use std::net::TcpListener;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // 환경변수 PORT 읽기, 없으면 기본 8000
    let port = env::var("PORT").unwrap_or("8000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(addr).expect("포트 바인딩 실패");
    println!("서버가 포트 {}에서 실행 중입니다", port);

    for stream in listener.incoming() {
        let mut stream = stream.expect("스트림 오류");

        let buf_reader = BufReader::new(&stream);

        let request_line = match buf_reader.lines().next() {
            Some(line_result) => match line_result {
                Ok(line) => line,
                Err(e) => {
                    eprintln!("요청 라인 읽기 실패: {}", e);
                    continue;
                }
            },
            None => {
                eprintln!("요청 라인이 없습니다");
                continue;
            }
        };

        println!("요청: {}", request_line);

        let response = "HTTP/1.1 200 OK\r\n\r\nHello, world from Render!";
        stream.write_all(response.as_bytes()).unwrap();
    }
}
