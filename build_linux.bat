REM Will place the binary into the "target" dir
docker run --rm -it -v "%cd%":/home/rust/src ekidd/rust-musl-builder cargo build --release