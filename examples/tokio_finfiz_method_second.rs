

use serde::Serialize;
use std::error::Error;
use std::thread;
use std::time::Duration;
use thirtyfour::{
    prelude::{ElementWaitable, WebDriverError},
    By, DesiredCapabilities, WebDriver, WebElement,
};
use url::Url;


fn main() -> color_eyre::Result<(),Box<dyn Error>>  {
    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(run())
}



    pub async fn run() -> color_eyre::Result<(),Box<dyn Error>> {
    

    let _place: &str ="Place";
    let _driver = initialize_driver().await?;
    let url = Url::parse("https://finviz.com")?;

    _driver.goto(url).await?;
    thread::sleep(Duration::from_secs(2));

    search_location(&_driver, _place).await?;
    thread::sleep(Duration::from_secs(2));

    scrape_all(_driver.clone()).await?;
    close_browser(_driver.clone()).await?;

    Ok(())
}

async fn close_browser(_driver: WebDriver) -> Result<(), Box<dyn Error>> {


    // Always explicitly close the browser.
    _driver.quit().await?;

    Ok(())
}

async fn scrape_all(_driver: WebDriver) -> Result<(), Box<dyn Error>> {

    Ok(())
}

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.maximize_window().await?;
    Ok(driver)
}

async fn search_location(_driver: &WebDriver, _place: &str) -> Result<(), WebDriverError> {
    // click_choose_place(driver).await?;

    // write_place(driver, place).await?;

    // click_search_button(driver).await?;

    Ok(())
}

