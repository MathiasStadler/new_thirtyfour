use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::time::Duration;
#[allow(unused_imports)]
use std::ptr::eq;

use thirtyfour::prelude::*;

const CLICK_ACTION:&str = "click-action";
const INPUT_ACTION:&str = "input";


const WEB_XPATH:&[&[&str]] = &[
     //No.,Action,FieldName,xpath
     // Action =>
     // click
     // input
     &["1",CLICK_ACTION,"accept","/html/body/div[1]/div/div/div/div[2]/div/button[3]"],
     &["2",CLICK_ACTION,"select toolbar","/html/body/table[1]/tbody/tr[1]/td/table/tbody/tr/td[1]/table/tbody/tr[2]/td/div/label/div/input"],
     &["3",INPUT_ACTION,"sendKeys and click","/html/body/table[1]/tbody/tr[1]/td/table/tbody/tr/td[1]/table/tbody/tr[2]/td/div/label/div/input"],
    
     ];

// main
fn main() -> color_eyre::Result<(),Box<dyn Error>>  {
    color_eyre::install()?;

    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(run())
}


const SCREENSHOT_BROWSER:&str = "screenshot_browser.png" ;

// run

pub async fn run() -> color_eyre::Result<(),Box<dyn Error>> {

let _caps = DesiredCapabilities::chrome();

let _driver = WebDriver::new("http://localhost:9515", _caps).await?;

_driver.goto("https://finviz.com/quote.ashx?t=TREX&p=d").await?;

screenshot_browser(_driver.clone(),SCREENSHOT_BROWSER).await?;

// MISSING Error handling
path_to_screenshot(_driver).await?;

// Always remember to close the session.
// driver.quit().await?;

Ok(())
}

//scrape_all
async fn path_to_screenshot(_driver: WebDriver) ->  color_eyre::Result<(),Box<dyn Error>> {
    
    //No.,Action,FieldName,xpath
    for field in 0 .. WEB_XPATH.len() {

        println!("No.   => {}",WEB_XPATH[field][0]);
        println!("Action => {}",WEB_XPATH[field][1]);
        println!("Field => {}",WEB_XPATH[field][2]);
        println!("XPath => {}",WEB_XPATH[field][3]);
        
        
        let _elem_form: WebElement = _driver
        .find(By::XPath(
            WEB_XPATH[field][3],
                   ))
        .await?;


    // if (WEB_XPATH[1] eq CLICK_ACTION){
        
    // // click
    // elem_form.click().await?;
    
    // } // end WEB_XPATH[1] eq CLICK_ACTION

    // if (WEB_XPATH[1] eq INPUT_ACTION){

    //     elem_form.send_keys("TREX").await?;
    //     // elem_form.click().await?;

    // }
    
    wait_seconds_of_browser(_driver.clone(),5).await?;
    }

    wait_seconds_of_browser(_driver.clone(),20).await?;

    Ok(())
}

async fn screenshot_browser(_driver: WebDriver, name : &str) -> color_eyre::Result<(),Box<dyn Error>> {

    //screenshot of browser windows
    // FROM HERE
    // https://stackoverflow.com/questions/60999624/trying-to-take-and-save-a-screenshot-of-a-specific-element-selenium-python-ch

    let screenshot = _driver.screenshot_as_png().await?;

    // FROM HERE  write to file
    // https://doc.rust-lang.org/std/fs/struct.File.html
    //let mut file = File::create("screenshot.png")?;
    let mut file = File::create(&name)?;
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