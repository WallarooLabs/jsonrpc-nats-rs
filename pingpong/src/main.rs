use clap::Parser;

mod client;
mod server;

mod pingpong;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, help = "NATS address", default_value = "localhost")]
    addr: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    Client {
        #[arg(help = "Ping text")]
        text: String,
        #[arg(help = "Ping count")]
        count: usize,
    },
    Server,
}

impl Cli {
    async fn dispatch(self) -> anyhow::Result<()> {
        self.command.dispatch(self.addr).await
    }
}

impl Command {
    async fn dispatch(self, addr: String) -> anyhow::Result<()> {
        match self {
            Self::Client { text, count } => client::client(addr, text, count).await,
            Self::Server => server::server(addr).await,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Cli::parse().dispatch().await
}
