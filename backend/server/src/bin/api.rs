use clap::{command, Parser, Subcommand};
use color_eyre::{eyre::Context, Help, Result};
use std::net::SocketAddr;
use tracing::{debug, error, info};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(
        short,
        long,
        default_value = "postgres://test@localhost/test",
        env = "API_DATABASE_URL"
    )]
    database_url: String,

    #[clap(short, long, default_value = "127.0.0.1:8070", env = "API_BIND")]
    bind: SocketAddr,

    #[clap(flatten)]
    verbosity: socialverse_server::logging::Verbosity,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// generate a session signing key
    GenKey,
}

async fn run() -> Result<()> {
    color_eyre::install()?;

    let use_dotenv = dotenvy::dotenv();

    let args = Cli::parse();

    socialverse_server::logging::setup(args.verbosity);

    if let Ok(path) = use_dotenv {
        debug!(target: "socialverse_server", dot_env_found = true, path = %path.to_string_lossy());
    } else {
        debug!(target: "socialverse_server", dot_env_found = false);
    }

    if let Some(command) = args.command {
        match command {
            Command::GenKey => {
                let mut rng = socialverse_crypto::new_rng();
                info!(target: "socialverse_server", "generating private key...");
                let (key, _) = socialverse_server::cli::gen_keys(&mut rng)?;
                println!("key.... {:?}", key);
                let path = "private_key.base64";
                std::fs::write(path, key.as_str())?;
                info!(target: "socialverse_server", path=path, "private key saved to disk");
                info!(target: "socialverse_server", "set API_PRIVATE_KEY environment variable with the content of the key in order to use it");
                return Ok(());
            }
        }
    }

    debug!(target: "socialverse_server", "loading signing keys");
    let signing_keys = socialverse_server::cli::load_keys()?;

    info!(target: "socialverse_server", database_url = args.database_url, "connecting to database");
    let db_pool = socialverse_query::AsyncConnectionPool::new(&args.database_url)
        .await
        .with_suggestion(|| "check database URL")
        .with_suggestion(|| "ensure correct database access rights")
        .with_suggestion(|| "make sure database exists")?;

    let state = socialverse_server::AppState {
        db_pool,
        signing_keys,
        rng: socialverse_crypto::new_rng(),
    };

    info!(target: "socialverse_server", bind_addr = %args.bind);

    let router = socialverse_server::router::new_router(state);

    let server = axum::Server::try_bind(&args.bind)
        .wrap_err_with(|| "server initialization error")
        .with_suggestion(|| "check bind address")
        .with_suggestion(|| "check if other services are using the same port")?;

    let server = server.serve(router.into_make_service());

    info!(target: "socialverse_server", "listening");

    if let Err(e) = server.await {
        error!(target: "socialverse_server", server_error = %e);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
