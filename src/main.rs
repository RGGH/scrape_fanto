use fantoccini::{ClientBuilder, Locator};
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("❌ failed to connect to WebDriver - have you started it in a seperate shell?\n");

    // Navigate to the target Wikipedia page
    c.goto("https://en.wikipedia.org/wiki/Foobar").await?;

    // Define the XPath for the specific section
    let xpaths = vec![
        "/html/body/div[2]/div/div[3]/main/div[3]/div[3]/div[1]/p[3]",
        "/html/body/div[2]/div/div[3]/main/div[3]/div[3]/div[1]/p[4]",
    ];

    let mut content = String::new();

    for xpath in xpaths {
        if let Ok(element) = c.find(Locator::XPath(xpath)).await {
            if let Ok(text) = element.text().await {
                content.push_str(&format!("{}\n\n", text));
            } else {
                eprintln!("Failed to retrieve text for XPath: {}", xpath);
            }
        } else {
            eprintln!("XPath not found: {}", xpath);
        }
    }

    // Write the refined extracted text to a file
    let mut file = File::create("xpath_extracted_text.txt").expect("failed to create file");
    file.write_all(content.as_bytes())
        .expect("failed to write to file");
    println!("Content saved to xpath_extracted_text.txt");

    c.close().await
}

