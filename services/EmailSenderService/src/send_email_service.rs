use crate::modules::Email;
use actix_web::{post, web, HttpResponse};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::Tls;
use lettre::transport::smtp::client::TlsParameters;
use lettre::{SmtpTransport, Transport};

#[post("/send_email")]
pub async fn send_email(email_request: web::Json<Email>) -> Result<HttpResponse, actix_web::Error> {
    let google_smtp_server = "smtp.gmail.com";
    let account_username = "proj.mail12@gmail.com";
    let account_password = "tpop pyhb cwsn rkqi";

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
            log::error!("Parsing Error -> {}", e);
            actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
        })?)
        .to(email_request.to.parse().map_err(|e| {
            log::error!("Parsing Error -> {}", e);
            actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
        })?)
        .subject(&email_request.subject)
        .body(email_request.body.clone())
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
        })?
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
    smtp_transport_mailer.send(message).map_err(|e| {
        log::error!("Error to send message -> {}", e);
        actix_web::error::ErrorInternalServerError("ErrorInternalServerError")
    })?;
    Ok(HttpResponse::Ok().body(format!(
        "Message to: {} successfully sent!",
        email_request.to
    )))
}
