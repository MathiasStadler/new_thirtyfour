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

let driver = WebDriver::new("http://localhost:9515", caps).await?;
driver.goto("https://www.rust-lang.org/").await?;
// Always remember to close the session.
// driver.quit().await?;
Ok(())
}