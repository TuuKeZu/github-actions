use rustdate::{update::UpdateBuilder, utility::OsType};

const VERSION: &str = env!("CARGO_PKG_VERSION");

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    UpdateBuilder::new()
        .set_verbal(true)
        .set_github_user("TuuKeZu")
        .set_github_repo("github-actions")
        .set_binary_path(OsType::Windows, "x86_64-pc-windows-gnu.zip")
        .set_binary_path(OsType::Linux, "x86_64-apple-darwin.zip")
        .check_for_updates()
        .await?;

    println!("Hello.rs v{VERSION}");
    println!("-------------------");
    println!("> Hello, world!");


    // terminal shouldn't terminate on close
    println!("Press any key to continue...");
    std::io::stdin().read_line(&mut String::new()).unwrap();

    Ok(())
}
