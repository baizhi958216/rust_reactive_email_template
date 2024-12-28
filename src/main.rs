extern crate lettre;

use askama::Template;
use dotenv::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(Template)]
#[template(path = "index.html")]

struct MailTemplate<'a> {
    company_name: &'a str,
}

fn main() {
    dotenv().ok();
    // 发件人邮箱
    let mine_email = std::env::var("mine_email").unwrap_or_else(|_| "QAQ...".to_string());

    // smtp服务器
    let smtp_server = std::env::var("smtp_server").unwrap_or_else(|_| "QwQ...".to_string());

    // smtp授权码
    let smtp_password = std::env::var("smtp_password").unwrap_or_else(|_| "QvQ...".to_string());

    // 收件人邮箱
    let email_receiver = "";

    // 邮件标题
    let email_title = "SihaSiha: 哇哦:D 是1大鸽鸽ψ(｀∇´)ψ";

    // 邮件内容
    let mail_body = MailTemplate {
        company_name: "AowuAowu",
    };

    let email = Message::builder()
        .from(mine_email.parse().unwrap())
        .reply_to(mine_email.parse().unwrap())
        .to(email_receiver.parse().unwrap())
        .subject(email_title)
        .header(ContentType::TEXT_HTML)
        .body(String::from(mail_body.render().unwrap()))
        .unwrap();

    let creds = Credentials::new(mine_email.to_owned(), smtp_password.to_owned());

    let mailer = SmtpTransport::relay(smtp_server.as_str())
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
