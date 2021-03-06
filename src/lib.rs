use fantoccini::error::NewSessionError;
use fantoccini::{Client, ClientBuilder};

pub enum Browser {
    Chrome,
    Firefox,
}

pub async fn connect(
    browser: Browser,
    headless: bool,
    host: Option<&str>,
    port: Option<u16>,
) -> Result<Client, NewSessionError> {
    match browser {
        Browser::Chrome => {
            // https://chromedriver.chromium.org/capabilities
            let mut caps = serde_json::map::Map::new();
            let args = if headless {
                serde_json::json!([
                    "--headless",
                    "--disable-gpu",
                    "--no-sandbox",
                    "--disable-dev-shm-usage"
                ])
            } else {
                serde_json::json!(["--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage"])
            };
            let opts = serde_json::json!({
                "args": args,
                "binary":
                    if std::path::Path::new("/usr/bin/chromium-browser").exists() {
                        // on Fedora and Ubuntu, it's called chromium-browser
                        "/usr/bin/chromium-browser"
                    } else {
                        // elsewhere, it's just called chromium
                        "/usr/bin/chromium"
                    }
            });
            caps.insert("goog:chromeOptions".to_string(), opts.clone());

            ClientBuilder::rustls()
                .capabilities(caps)
                .connect(&make_url(host, port.unwrap_or(9515)))
                .await
        }
        Browser::Firefox => {
            // https://developer.mozilla.org/en-US/docs/Web/WebDriver/Capabilities/firefoxOptions
            let mut caps = serde_json::map::Map::new();
            let args = if headless {
                serde_json::json!(["--headless"])
            } else {
                serde_json::json!([])
            };
            let opts = { serde_json::json!({ "args": args }) };
            caps.insert("moz:firefoxOptions".to_string(), opts.clone());
            ClientBuilder::rustls()
                .capabilities(caps)
                .connect(&make_url(host, port.unwrap_or(4444)))
                .await
        }
    }
}

fn make_url(host: Option<&str>, port: u16) -> String {
    format!("http://{}:{}", host.unwrap_or("localhost"), port)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn connect() -> Result<(), Box<dyn std::error::Error>> {
        let client = super::connect(Browser::Chrome, true, None, None).await?;

        client.goto("https://www.google.com/").await?;

        Ok(())
    }
}
