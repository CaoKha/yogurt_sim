pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}


#[derive(Serialize)]
pub struct MyObject {
    username: String,
    favorite_language: String,
}
