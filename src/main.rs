use dotenv::dotenv;
use tide::Request;
use tide::prelude::*;
use log::info;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    env_logger::init();
    dotenv().ok();
    
    let bind = std::env::var("BIND").expect("BIND must be set.");    

    info!("Server running at http://{}", bind);

    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen(bind).await?;
    Ok(())    

}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}
