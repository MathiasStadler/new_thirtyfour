// use serde::Serialize;
use std::error::Error;
// use std::thread;
// use std::time::Duration;
// use std::fs::File;
// use std::io::prelude::*;



// use csv::Writer;
use thirtyfour::{
    // prelude::{ElementWaitable, WebDriverError},
    prelude::{WebDriverError},
    By,
    DesiredCapabilities,
    WebDriver,
    // WebElement,
};
use url::Url;

fn main() -> color_eyre::Result<(),Box<dyn Error>>  {
    color_eyre::install()?;

    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(run())
}

pub async fn run() -> color_eyre::Result<(),Box<dyn Error>> {
    
    let _driver = initialize_driver().await?;
    let _url = Url::parse("https://wikipedia.org")?;
    
    _driver.goto(_url).await?;

    let elem_form = _driver.find(By::Id("search-form")).await?;

    // Find element from element.
    let elem_text = elem_form.find(By::Id("searchInput")).await?;

    // Type in the search terms.
    elem_text.send_keys("selenium").await?;

    // Click the search button.
    let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    _driver.find(By::ClassName("firstHeading")).await?;
    
    let _result =  _driver.title().await?;

    println!("{}",_result);
    // match _result.await? {
    // Ok(_title) => println!("Page title {:}", _title)
    // Err(_) => println!("error"),
    // }

    assert_eq!(_driver.title().await?, "Selenium - Wikipedia");

    // Always explicitly close the browser. There are no async destructors.
    let _ = _driver.quit().await?;
    Ok(())
}//end of main

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    // let caps = DesiredCapabilities::chrome();
    
    let mut _caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
    _caps.add_chrome_arg("--headless")?;
    _caps.add_chrome_arg("--no-sandbox")?;
    _caps.add_chrome_arg("--disable-dev-shm-usage")?;
    
    let _driver = WebDriver::new("http://localhost:9515", _caps).await?;
    // driver.maximize_window().await?;
    Ok(_driver)
}