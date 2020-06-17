# Rust : Rocket Test !! 

> 이 문서는 Rust 를 이용한 CRUD API를 개발하면서 작성한 테스트 코드에 대한 글입니다.

## Create the applications

이 프로젝트에서 사용한 Rocket은 Rust Nightly에서 동작합니다. 혹시라도 stable을 사용한다면 Rust Nightly를 디폴트로 적용해줍시다 

```shell
$ rustup default nightly
$ rustup update && cargo update
cargo 1.45.0-nightly (40ebd5220 2020-06-01)
rustc 1.46.0-nightly (feb3536eb 2020-06-09)
```

바이너리 프로젝트를 생성하고 의존성을 추가해줍시다. 
```shell
$ cargo new rocket-test --bin && cd rocket-test

```

디렉토리를 생성하면 `Cargo.toml`라는 파일이 생겼을텐데, 이 파일은 Node 에서의 package.json과 비슷한 역할을 한다고 보시면 됩니다. 

이 부분에 우리가 사용할 Rocket의 dependency를 추가해줍시다 .. 

```rust
// Cargo.toml
[dependencies]
rocket = "0.4.5"
```

이제 `src/main.rs`를 수정할 차례입니다. 프로젝트 생성 직후에는 `Hello World` 프로그램이 들어있습니다. 이걸 약간 고쳐줍시다.

```rust
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, Rust!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
```

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.77s
     Running `.\rocket_getting_started.exe`
Configured for development.
    => address: localhost
    => port: 8000
    => log: normal
    => workers: 16
    => secret key: generated
    => limits: forms = 32KiB
    => keep-alive: 5s
    => tls: disabled
Mounting /:
    => GET / (hello)
Rocket has launched from http://localhost:8000

```

![](https://images.velog.io/images/eeeclipse/post/11bf48aa-8565-441c-80a4-bd585b75007a/image.png)

## Reference 
[Rocket])(https://rocket.rs/)
