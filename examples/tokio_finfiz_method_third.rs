

use serde::Serialize;
use std::error::Error;
use std::thread;
use std::time::Duration;
use thirtyfour::{
    prelude::{ElementWaitable, WebDriverError},
    By, DesiredCapabilities, WebDriver, WebElement,
};
use url::Url;
use std::fs::File;
use std::io::prelude::*;


const WEB_XPATH:&[&[&str]] = &[
     //No.,FieldName,xpath        
     &["1","accept","/html/body/div[1]/div/div/div/div[2]/div/button[3]"],
     &["2","screener","/html/body/table[2]/tbody/tr/td/table/tbody/tr/td[3]/a"],
     &["3","screener all view","/html/body/div[4]/table/tbody/tr[2]/td/div/div[2]/div[5]"],
     &["4","select exchange","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[1]/td[2]/select/option[3]"],
    //  &["5","select Market Cap","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[1]/td[2]/select/option[3]"],
    //  &["6","select Option/Short","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[8]/td[10]/select/option[2]"],
    //  &["7","200-Day Simple Moving Average","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[10]/td[8]/select/option[12]"],
    //  &["8","sma_over_50_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[10]/td[6]/select/option[8]"],
    //  &["9","sma_over_20_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[10]/td[4]/select/option[8]"],
    //  &["10","price_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[13]/td[8]/select/option[39]"],
    //  &["11","pattern_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[13]/td[8]/select/option[39]"],
    //  &["12","peg_over_one_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[2]/td[8]/select/option[7]"],
    //  &["13","eps_year_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[3]/td[8]/select/option[3]"],
    //  &["14","eps_qtr_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[4]/td[8]/select/option[3]"],
    //  &["15","peg_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[2]/td[8]/select/option[7]"],
    //  &["16","beta_xpath","/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[12]/td[6]/select/option[7]"],
    // &["r1","colum_name","/html/body/div[4]/table/tbody/tr[4]/td/div/table/tbody/tr[5]/td/table/tbody/tr/td/table/thead"],
    // &["r2","screener_result","/html/body/div[4]/table/tbody/tr[4]/td/div/table/tbody/tr[5]/td/table/tbody/tr/td/table/tbody/tr"],
    ];


fn main() -> color_eyre::Result<(),Box<dyn Error>>  {
    color_eyre::install()?;

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
    screenshot_browser(_driver.clone()).await?;
    save_result_table(_driver.clone()).await?;
    // close_browser(_driver.clone()).await?;

    Ok(())
}

async fn close_browser(_driver: WebDriver) -> color_eyre::Result<(),Box<dyn Error>> {

    // Always explicitly close the browser.
    _driver.quit().await?;

    Ok(())
}

async fn screenshot_browser(_driver: WebDriver) -> color_eyre::Result<(),Box<dyn Error>> {

    //screenshot of browser windows
    // FROM HERE
    // https://stackoverflow.com/questions/60999624/trying-to-take-and-save-a-screenshot-of-a-specific-element-selenium-python-ch

    let screenshot = _driver.screenshot_as_png().await?;

    // FROM HERE  write to file
    // https://doc.rust-lang.org/std/fs/struct.File.html
    let mut file = File::create("foo.png")?;
    file.write_all(&screenshot)?;

    // println!("Screenshot of browser windows => {:?} ",screenshot);
    Ok(())
}


async fn wait_seconds_of_browser(_driver: WebDriver,waiting_period:u64) -> color_eyre::Result<(),Box<dyn Error>> {

    // wait for page already load
println!("Status driver => {:?}", _driver.status().await?);

tokio::time::sleep(Duration::from_secs(waiting_period)).await;

Ok(())
}

async fn scrape_all(_driver: WebDriver) ->  color_eyre::Result<(),Box<dyn Error>> {

    for field in 0 .. WEB_XPATH.len() {

        println!("No.   => {}",WEB_XPATH[field][0]);
        println!("Field => {}",WEB_XPATH[field][1]);
        println!("XPath => {}",WEB_XPATH[field][2]);
        let elem_form: WebElement = _driver
        .find(By::XPath(
            WEB_XPATH[field][2],
                   ))
        .await?;
    elem_form.click().await?;
    wait_seconds_of_browser(_driver.clone(),5).await?;
    }

    wait_seconds_of_browser(_driver.clone(),20).await?;

    Ok(())
}


//save_result_table
async fn save_result_table(_driver: WebDriver) ->  color_eyre::Result<(),Box<dyn Error>> {

    const SAVE_TABLE:&[&[&str]] = &[
     //No.,FieldName,xpath        
     &["t1","colum_name","/html/body/div[4]/table/tbody/tr[4]/td/div/table/tbody/tr[5]/td/table/tbody/tr/td/table/thead"],
     &["t2","No.:","/html/body/div[4]/table/tbody/tr[4]/td/div/table/tbody/tr[5]/td/table/tbody/tr/td/table/tbody/tr"],
     ];

     
        //debug
        let field=0;
        println!("No.   => {}",SAVE_TABLE[field][0]);
        println!("Field => {}",SAVE_TABLE[field][1]);
        println!("XPath => {}",SAVE_TABLE[field][2]);

        let thead_rows_vec:Vec<WebElement> = _driver.find_all(By::XPath(SAVE_TABLE[field][2])).await?;
        for thead_row in thead_rows_vec{
            println!("row => {}",thead_row);
            println!("row Name: {}",thead_row.tag_name().await?);
            println!("row Text: {}",thead_row.text().await?);

        //debug
        let field=1;
        println!("No.   => {}",SAVE_TABLE[field][0]);
        println!("Field => {}",SAVE_TABLE[field][1]);
        println!("XPath => {}",SAVE_TABLE[field][2]);
        
        let thead_cell_vec:Vec<WebElement> = thead_row.find_all(By::XPath(SAVE_TABLE[field][2])).await?;
        
        for thead_cell in thead_cell_vec {
            println!("cell => {}",thead_cell);
            println!("cell Name: {}",thead_cell.tag_name().await?);
            println!("cell Text: {}",thead_cell.text().await?);
        }
        }

    Ok(())
}

// /html/body/div[4]/table/tbody/tr[4]/td/div/table/tbody/tr[5]/td/table/tbody/tr/td/table/tbody/tr[1]/td[1]/a

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

