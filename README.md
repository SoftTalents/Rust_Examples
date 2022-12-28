## Usage 

To run this application, run the following command in shell.
```sh
cargo run -- searchstring example-file.txt
```
To test, run the below.
```sh
cargo test
```
You can set environment variable for search.
```sh
IGNORE_CASE=1 cargo run -- to poem.txt
```
To output application result to specific file, run the following command.
```sh
cargo run > output.txt
```
