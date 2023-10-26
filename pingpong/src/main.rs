use clap::Parser;
use jsonrpc_nats::Nats;
use tracing_subscriber::fmt;
use tracing_subscriber::EnvFilter;

mod client;
mod server;

mod count;
mod pingpong;
mod simple;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(
        long,
        help = "NATS address",
        default_value = "nats://localhost:4222",
        alias = "addr"
    )]
    addrs: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    Client {
        #[clap(subcommand)]
        method: Method,
    },
    Server,
}

#[derive(Debug, clap::Subcommand)]
enum Method {
    Count,
    Ping {
        #[arg(help = "Ping text")]
        text: String,
        #[arg(help = "Ping count")]
        count: usize,
    },
    Simple,
}

impl Cli {
    async fn dispatch(self) -> anyhow::Result<()> {
        self.command.dispatch(self.addrs).await
    }
}

impl Command {
    async fn dispatch(self, addrs: String) -> anyhow::Result<()> {
        match self {
            Self::Client { method } => client::client(addrs, method).await,
            Self::Server => server::server(addrs).await,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();
    Cli::parse().dispatch().await
}
