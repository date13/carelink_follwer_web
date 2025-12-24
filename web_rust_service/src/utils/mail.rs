use anyhow::Error;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tracing::info;

pub struct EmailService {
  smtp_server: String,
  smtp_port: u16,
  username: String,
  password: String,
  from: String,
  pub to: String,
}

impl EmailService {
  pub fn new(
    smtp_server: String,
    smtp_port: u16,
    username: String,
    password: String,
    from: String,
    to: String,
  ) -> Self {
    Self {
      smtp_server,
      smtp_port,
      username,
      password,
      from,
      to,
    }
  }

  /// 发送纯文本邮件
  pub async fn send_text_email(
    &self,
    to: &str,
    subject: &str,
    body: &str,
  ) -> Result<(), Error> {
    let email = Message::builder()
        .from(self.from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    let creds = Credentials::new(self.username.clone(), self.password.clone());

    let mailer = SmtpTransport::relay(&self.smtp_server)?
        .port(self.smtp_port)
        .credentials(creds)
        .build();

    // 发送邮件（异步）
    mailer.send(&email)?;
    info!("HTML email sent successfully to: {}", to);
    Ok(())
  }

  // 发送 HTML 邮件
  // pub async fn send_html_email(
  //   &self,
  //   to: &str,
  //   subject: &str,
  //   html_body: &str,
  //   text_body: Option<&str>,
  // ) -> Result<(), Error> {
  //   let mut email_builder = Message::builder()
  //       .from(self.from.parse()?)
  //       .to(to.parse()?)
  //       .subject(subject);
  //
  //   let email = if let Some(text) = text_body {
  //     // 多部分邮件（HTML + 纯文本）
  //     email_builder.multipart(
  //       lettre::message::MultiPart::alternative()
  //           .singlepart(
  //             lettre::message::SinglePart::builder()
  //                 .header(lettre::message::header::ContentType::TEXT_PLAIN)
  //                 .body(text.to_string()),
  //           )
  //           .singlepart(
  //             lettre::message::SinglePart::builder()
  //                 .header(lettre::message::header::ContentType::TEXT_HTML)
  //                 .body(html_body.to_string()),
  //           ),
  //     )?
  //   } else {
  //     // 仅 HTML
  //     email_builder.body(html_body.to_string())?
  //   };
  //
  //   let creds = Credentials::new(self.username.clone(), self.password.clone());
  //
  //   let mailer = SmtpTransport::relay(&self.smtp_server)?
  //       .port(self.smtp_port)
  //       .credentials(creds)
  //       .build();
  //
  //   mailer.send(&email)?;
  //   info!("HTML email sent successfully to: {}", to);
  //   Ok(())
  // }
}