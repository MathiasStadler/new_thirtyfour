use std::error::Error;

use thirtyfour::prelude::*;


fn main() -> color_eyre::Result<(),Box<dyn Error>>  {
    color_eyre::install()?;

    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(run())
}

pub async fn run() -> color_eyre::Result<(),Box<dyn Error>> {

let caps = DesiredCapabilities::chrome();
// NOTE: this assumes you have a WebDriver compatible server running
//       at http://localhost:4444
//       e.g. `geckodriver -p 4444`
let driver = WebDriver::new("http://localhost:9515", caps).await?;
driver.goto("https://www.rust-lang.org/").await?;
// Always remember to close the session.
driver.quit().await?;
Ok(())
}