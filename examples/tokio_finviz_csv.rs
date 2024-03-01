//! Requires chromedriver running on port 9515:
//!
//!     chromedriver --port=9515
//!
//! Run as follows:
//!
//!     cargo run --example tokio_async


use thirtyfour::prelude::*;
use tokio::time::*;

fn main() -> color_eyre::Result<()> {
    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(run())
}



/* function `set_filter_items` is never used
async fn set_filter_items(driver: &WebDriver,xpath_path : &str)  -> color_eyre::Result<()> {
    //color_eyre::install()?;

    println!("str {}",xpath_path);
    //let elem_exchange_nyse: WebElement = driver.find(By::XPath(exchange_nyse_xpath)).await?;
    //elem_exchange_nyse.click().await?;
    driver.find(By::XPath(xpath_path)).await?.click().await?;

    Ok(())

}
*/

async fn run() -> color_eyre::Result<()> {
    // The use of color_eyre gives much nicer error reports, including making
    // it much easier to locate where the error occurred.
    color_eyre::install()?;

    let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;

    //let caps = DesiredCapabilities::firefox();
    let driver: WebDriver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to target page 
    driver.goto("https://finviz.com").await?;


    //wait 5 second
    //from here
    //https://dev.to/stevepryde/using-selenium-with-rust-aca
    tokio::time::sleep(Duration::from_secs(5)).await;

    // site title
    println!("Title = {}", driver.title().await?);
   

    let elem_form: WebElement = driver
        .find(By::XPath(
            "/html/body/div[1]/div/div/div/div[2]/div/button[3]",
        ))
        .await?;
    elem_form.click().await?;

    // wait for page already load
    println!("Status driver => {:?}", driver.status().await?);

    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // select screener
    println!("Click on screener");

    // click on button screener
    let elem_screener: WebElement = driver
        .find(By::XPath(
            "/html/body/table[2]/tbody/tr/td/table/tbody/tr/td[3]/a",
        ))
        .await?;
    elem_screener.click().await?;

    println!("Status driver => {:?}", driver.status().await?);

    //wait for screener
    tokio::time::sleep(Duration::from_secs(3)).await;

    // select screener all view
    let screener_all_view_xpath: &str = "/html/body/div[4]/table/tbody/tr[2]/td/div/div[2]/div[5]";

    let elem_screener_all: WebElement = driver.find(By::XPath(screener_all_view_xpath)).await?;
    elem_screener_all.click().await?;

    println!("Status driver => {:?}", driver.status().await?);
    
    //wait for screener
    tokio::time::sleep(Duration::from_secs(3)).await;

    driver.maximize_window().await?;

    println!("Status driver => {:?}", driver.status().await?);

    //wait for screener
    tokio::time::sleep(Duration::from_secs(3)).await;

    println!("Status driver => {:?}", driver.status().await?);
    
    //select exchange
    let exchange_nyse_xpath: &str =
        "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[1]/td[2]/select/option[3]";
    let elem_exchange_nyse: WebElement = driver.find(By::XPath(exchange_nyse_xpath)).await?;
    elem_exchange_nyse.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //select Market Cap.
    let market_cap_xpath: &str =
        "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[1]/td[2]/select/option[3]";
    let elem_market_cap: WebElement = driver.find(By::XPath(market_cap_xpath)).await?;
    elem_market_cap.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //select Option/Short => Optionable
    let market_cap_xpath: &str =
        "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[8]/td[10]/select/option[2]";
    let elem_market_cap: WebElement = driver.find(By::XPath(market_cap_xpath)).await?;
    elem_market_cap.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //200-Day Simple Moving Average
    let over_200_xpath: &str = "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[10]/td[8]/select/option[12]";
    let elem_over_200: WebElement = driver.find(By::XPath(over_200_xpath)).await?;
    elem_over_200.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //50-Day Simple Moving Average
    let sma_over_50_xpath: &str =
        "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[10]/td[6]/select/option[8]";
    let elem_sma_over_50: WebElement = driver.find(By::XPath(sma_over_50_xpath)).await?;
    elem_sma_over_50.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //20-Day Simple Moving Average
    let sma_over_20_xpath: &str =
    "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[10]/td[4]/select/option[8]";
    let elem_sma_over_20: WebElement = driver.find(By::XPath(sma_over_20_xpath)).await?;
    elem_sma_over_20.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    // price
    let price_xpath: &str = "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[13]/td[8]/select/option[39]";
    let elem_price: WebElement = driver.find(By::XPath(price_xpath)).await?;
    elem_price.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //Pattern channel up
    let pattern_xpath: &str = "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[13]/td[8]/select/option[39]";
    let elem_pattern: WebElement = driver.find(By::XPath(pattern_xpath)).await?;
    elem_pattern.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //PEG over one
    let peg_over_one_xpath: &str =
        "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[2]/td[8]/select/option[7]";
    let elem_peg_over_one: WebElement = driver.find(By::XPath(peg_over_one_xpath)).await?;
    elem_peg_over_one.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //EPS year positive
    let eps_year_xpath: &str =
            "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[3]/td[8]/select/option[3]";
    let elem_eps_year: WebElement = driver.find(By::XPath(eps_year_xpath)).await?;
    elem_eps_year.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //EPS qtr positive
    let eps_qtr_xpath: &str =
        "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[4]/td[8]/select/option[3]";
    let elem_eps_qtr: WebElement = driver.find(By::XPath(eps_qtr_xpath)).await?;
    elem_eps_qtr.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //PEG over 1
    let peg_xpath: &str =
        "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[2]/td[8]/select/option[7]";
    let elem_peg: WebElement = driver.find(By::XPath(peg_xpath)).await?;
    elem_peg.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;

    //BETA 
    let beta_xpath: &str =
        "/html/body/div[4]/table/tbody/tr[3]/td/div/form/table/tbody/tr[12]/td[6]/select/option[7]";
    let elem_beta: WebElement = driver.find(By::XPath(beta_xpath)).await?;
    elem_beta.click().await?;

    //wait for refresh
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    //wait for debug not necessary
    tokio::time::sleep(Duration::from_secs(20)).await;

    // thead
    let table_result_thead_xpath = "/html/body/div[4]/table/tbody/tr[4]/td/div/table/tbody/tr[5]/td/table/tbody/tr/td/table/thead" ;
    let thead_rows_vec:Vec<WebElement> = driver.find_all(By::XPath(table_result_thead_xpath)).await?;

        for thead_row in thead_rows_vec{
            println!("row => {}",thead_row);
            println!("row Name: {}",thead_row.tag_name().await?);
            println!("row Text: {}",thead_row.text().await?);

            let thead_cell_vec:Vec<WebElement> = thead_row.find_all(By::XPath("/td")).await?;
        
        for thead_cell in thead_cell_vec {
            println!("cell => {}",thead_cell);
            println!("cell Name: {}",thead_cell.tag_name().await?);
            println!("cell Text: {}",thead_cell.text().await?);
        }
        }



    let table_result_xpath: &str="/html/body/div[4]/table/tbody/tr[4]/td/div/table/tbody/tr[5]/td/table/tbody/tr/td/table/tbody/tr";
    let tbody_rows_vec:Vec<WebElement> = driver.find_all(By::XPath(table_result_xpath)).await?;
    

    for tbody_row in tbody_rows_vec {
        println!("row => {}",tbody_row);
        println!("row Name => {}",tbody_row.tag_name().await?);
        println!("row Text => {}",tbody_row.text().await?);

        let tbody_cell_vec:Vec<WebElement> = tbody_row.find_all(By::XPath("/td")).await?;
        
        for tbody_cell in tbody_cell_vec {
            println!("cell => {}",tbody_cell);
            println!("cell Name: {}",tbody_cell.tag_name().await?);
            println!("cell Text: {}",tbody_cell.text().await?);
        }
    }

    

    // get result
    // let result_xpath: &str="/html/body/div[4]/table/tbody/tr[4]/td/div/table/tbody/tr[3]/td/div/div/div[1]";
    // let elem_result: Result<WebElement, std::num::ParseIntError> = driver.find(By::XPath(result_xpath)).await?;
    // println!("Ticker {}", elem_result);

    //from here
    //https://docs.rs/thirtyfour/latest/thirtyfour/struct.WebElement.html
    // let html: String = elem_result.inner_html().await?;
    // println!("Inner HTML =>{}", html);

    //write to file
    // fs::write("/home/trapapa/selenium_output.txt", html).expect("Unable to write file");

    /*
    let package = parser::parse("<root>hello</root>").expect("failed to parse XML");
    let document = package.as_document();

    let value = evaluate_xpath(&document, "/root").expect("XPath evaluation failed");

    println!("Result XPATH {}", value.string());
    */

    Ok(())
}
