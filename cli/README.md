# Cli

Rust 커뮤니티는 `main` 리펙토링에 대해 아래와 같은 가이드라인을 제공하고 있습니다.

1. 당신의 프로그램을 `main.rs` 과 `lib.rs` 로 나누고 프로그램의 로직을 `lib.rs` 으로 옮깁니다.
2. 커맨드라인 파싱 로직이 크지 않으면, `main.rs` 에 남겨둬도 됩니다.
3. 커맨드라인 파싱 로직이 복잡해지기 시작할거 같으면, `main.rs` 에서 추출해서 `lib.rs` 로 옮기세요.
4. 이런 절차를 통해 `main()` 함수에는 다음의 핵심 기능들만 남아있어야 합니다.
    1. 인자 값들로 커맨드라인을 파싱하는 로직 호출
    2. 다른 환경들 설정
    3. `lib.rs`의 `run` 함수 호출
    4. `run`이 에러를 리턴하면, 에러 처리.


- run cli
```js
# cargo run -- <키워드> <읽을파일이름>
$ IGNORE_CASE=1 cargo run -- dust poem.txt
$ cargo run -- rust poem.txt
```

- run test

```
cargo test
```