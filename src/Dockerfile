# 빌드 단계: Rust 애플리케이션 컴파일
FROM rust:1.75 as builder

# 작업 디렉토리 설정
WORKDIR /usr/src/rust_api_server_ex1

# 의존성 캐싱을 위해 Cargo.toml과 Cargo.lock 먼저 복사
COPY Cargo.toml Cargo.lock ./

# 의존성 빌드 (빈 src로 캐싱 활용)
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

# 실제 소스 코드 복사
COPY src ./src

# 애플리케이션 빌드
RUN cargo build --release

# 실행 단계: 슬림 이미지로 전환
FROM debian:bookworm-slim

# SQLite 설치
RUN apt-get update && apt-get install -y libsqlite3-0 && rm -rf /var/lib/apt/lists/*

# 작업 디렉토리 설정
WORKDIR /app

# 빌드된 바이너리 복사
COPY --from=builder /usr/src/rust_api_server_ex1/target/release/rust_api_server_ex1 .

# 데이터베이스 파일을 저장할 볼륨 디렉토리 설정
VOLUME /app/data

# 실행 명령어
CMD ["./rust_api_server_ex1"]