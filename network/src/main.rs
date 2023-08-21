use std::error::Error;

fn do_something() {}

#[tokio::main]
// Result<(), Box<dyn Error>>型は、
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.com/";
    let response = reqwest::get(url).await?;

    let content = response.text().await?;
    print!("{}", content);

    for _i in 0..100 {
        do_something();
    }

    Ok(())
}
