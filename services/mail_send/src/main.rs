use crate::structures::Email;
use actix_web::{post, web, HttpResponse};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::Tls;
use lettre::transport::smtp::client::TlsParameters;
use lettre::{SmtpTransport, Transport};

#[post("/send_email")]
pub async fn send_email(email_request: web::Json<Email>) -> Result<HttpResponse, actix_web::Error> {
    let google_smtp_server = "smtp.gmail.com"; // Сервак который будем имеет прикол быть посредником между всеми операциями
    let account_username = "proj.mail12@gmail.com"; // Я думаю понятно что это значит.
    let account_password = "tpop pyhb cwsn rkqi"; // нет смысла че то объяснять библеотка заходит и отправляет че надо

    let message = create_message(account_username, &email_request)?;
    let credentials = create_credentials(account_username, account_password);
    let smtp_transport_mailer = create_transport_mailer(google_smtp_server, credentials)?;
    let http_response = send_mail(&smtp_transport_mailer, &message, &email_request).await?;
    Ok(http_response)
}

fn create_message(
    account_username: &str,
    email_request: &web::Json<Email>,
) -> Result<lettre::Message, actix_web::Error> {
    lettre::Message::builder()
        .from(account_username.parse().map_err(|e| {
            //откуда прибудет наше сообщения
            log::error!("Parsing Error -> {}", e);
            actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
        })?)
        .to(email_request.to.parse().map_err(|e| {
            // адрес доставки нашего опозиционного письма
            log::error!("Parsing Error -> {}", e);
            actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
        })?)
        .subject(&email_request.subject) // Любимый субждект, я описал кто он и что из себя представляет в Донецком обществе в полях структуры.
        .body(email_request.body.clone()) // дружбан субджекта  так же описан в полях структуры
        .map_err(|e| {
            log::error!("Error to create responder message -> {}", e);
            actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
        })
}

fn create_credentials(account_username: &str, account_password: &str) -> Credentials {
    Credentials::new(account_username.to_string(), account_password.to_string())
}

fn create_transport_mailer(
    google_smtp_server: &str,
    credentials: Credentials,
) -> Result<SmtpTransport, actix_web::Error> {
    Ok(SmtpTransport::relay(google_smtp_server)
        .map_err(|e| {
            log::error!("Error to crate smtp_mailer -> {}", e);
            actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
        })? // создаем меилер который отправит письмо!
        .port(587)
        .credentials(credentials)
        .tls(Tls::Required(
            TlsParameters::new("smtp.gmail.com".to_string()).map_err(|e| {
                log::error!("Error to crate tls connection -> {}", e);
                actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
            })?,
        ))
        .build())
}

async fn send_mail(
    smtp_transport_mailer: &SmtpTransport,
    message: &lettre::Message,
    email_request: &Email,
) -> Result<HttpResponse, actix_web::Error> {
    match smtp_transport_mailer.send(message) {
        // Результат всего всего всего
        Ok(_) => {
            Ok(HttpResponse::Ok().body(format!("Email sent successfully to {}", email_request.to)))
        }
        Err(e) => {
            println!("{}", e);
            Ok(HttpResponse::InternalServerError().body(format!(
                "Error to send email to {}, error -> {}",
                email_request.to, e
            )))
        }
    }
}
