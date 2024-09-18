#[derive(serde::Serialize, Clone, Debug)]
pub enum Message {
    Text{user_id: String, content: String, date: u32},
    Image{url: String},
    File{url: String}
}
