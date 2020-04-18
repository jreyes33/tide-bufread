use async_std::fs::{metadata, File};
use async_std::io::BufReader;
use tide::http_types::{headers, StatusCode};
use tide::Response;

const FILE_NAME: &str = "large.txt"; // Try with "small.txt" as well.

fn main() {
    async_std::task::block_on(async {
        let mut app = tide::new();
        app.at("/").get(|_| async {
            let size = metadata(FILE_NAME).await.unwrap().len();
            let file = File::open(FILE_NAME).await.unwrap();
            Response::new(StatusCode::Ok)
                .body(BufReader::new(file))
                .set_header(headers::CONTENT_LENGTH, size.to_string())
                .set_header(headers::CONTENT_TYPE, "text/plain; charset=utf-8")
        });
        app.listen("127.0.0.1:8080").await.unwrap();
    })
}
