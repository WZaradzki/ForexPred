use async_std::stream::StreamExt;
// use scraper::html;
use sqlx::mysql::MySqlPoolOptions;

#[derive(sqlx::FromRow, Debug)]
struct Currency {
    name: String,
    iso: String,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://forex:password@localhost/forex")
        .await?;

    // // query to add currency
    // let currency = sqlx::query("INSERT INTO currencies (id, name, iso) VALUES (?, ?, ?)")
    //     .bind("2f59347b-3524-4f33-b2e1-b96a13e93b05")
    //     .bind("USD")
    //     .bind("Dolar Amerykański")
    //     .execute(&pool)
    //     .await?;

    let usd_currency = sqlx::query_as::<_, Currency>("SELECT * FROM currencies WHERE iso = 'USD'")
        .fetch_one(&pool)
        .await?;

    let pl_currency = sqlx::query_as::<_, Currency>("SELECT * FROM currencies WHERE iso = 'PLN'")
        .fetch_one(&pool)
        .await?;

    let browser = headless_chrome::Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to("https://www.myfxbook.com/forex-market/currencies/USDPLN")
        .unwrap();

    let exchange_rate = tab
        .wait_for_element("#symbolAnalysisBid")
        .unwrap()
        .get_attribute_value("data-value")
        .unwrap();

    let insert_exchange_rate = sqlx::query("INSERT INTO currency_rates (from_currency_iso, to_currency_iso, rate) VALUES (?, ?, ?)")
        .bind(usd_currency.iso)
        .bind(pl_currency.iso)
        .bind(exchange_rate)
        .execute(&pool)
        .await?;

    Ok(())
}

// // define a custom data structure
// // to store the scraped data
// #[derive(Debug)]
// struct PokemonProduct {
//     url: String,
//     image: String,
//     name: String,
//     // price: String,
// }
// #[tokio::main]
// async fn main() -> Result<(), sqlx::Error> {
//     //     let pool = MySqlPoolOptions::new()
//     //     .max_connections(5)
//     //     .connect("postgres://postgres:password@localhost/test").await?;

//     // // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
//     // let row: (i64,) = sqlx::query_as("SELECT $1")
//     //     .bind(150_i64)
//     //     .fetch_one(&pool).await?;

//
// }
