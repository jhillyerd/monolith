use lettre::{AsyncTransport, Message};

pub async fn send_alert<T>(mailer: &T, subject: &str, body: &str) -> anyhow::Result<()>
where
    T: AsyncTransport + Send + Sync,
    <T as AsyncTransport>::Error: 'static + Send + Sync,
    <T as AsyncTransport>::Error: std::error::Error,
{
    let email = Message::builder()
        .from("monolith@bytemonkey.org".parse().unwrap())
        .to("james@bytemonkey.org".parse().unwrap())
        .subject(subject)
        .body(body.to_string())?;

    mailer.send(email).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use lettre::transport::stub::AsyncStubTransport;

    use super::*;

    #[tokio::test]
    async fn test_alert() {
        let subj = "testsubject";
        let text = "testtext";

        let mailer = AsyncStubTransport::new_ok();

        let result = send_alert(&mailer, subj, text).await.unwrap();
        assert_eq!(result, ());

        let msgs = mailer.messages().await;
        assert_eq!(msgs.len(), 1);

        let (_, content) = msgs.first().unwrap();
        assert!(content.contains(format!("Subject: {subj}").as_str()));
        assert!(content.contains(text));
    }
}
