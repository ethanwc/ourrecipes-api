struct NewComment<'a> {
    body: &'a str,
    author: i32,
    article: i32,
}

pub fn create(author: i32, slug: &str, body: &str) -> &'static str {
    "hi"
}
