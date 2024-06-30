use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Email {
    pub to: String,      // Куда идет отправка сообщений.
    pub subject: String, // Заголовок(та черная хуйня вначале письма)
    pub body: String,    // Тело запроса. Основаная инфа!
}
