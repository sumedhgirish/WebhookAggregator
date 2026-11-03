pub use log::*;

use tokio_util::sync::CancellationToken;
use wagg::db::Database;
use wagg::dispatcher::Dispatcher;
use wagg::listener::WebListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let mut db = Database::init().await?;
    let dispatcher = Dispatcher::init(db.clone());
    let doorbells = dispatcher.doorbells.clone();

    let stop = CancellationToken::new();
    let disp = tokio::spawn({
        let s = stop.clone();
        async move { dispatcher.run(s).await }
    });

    let listener = WebListener::init(db.clone(), doorbells);
    let served = listener.run().await;

    stop.cancel();
    let dispatched = disp.await;

    db.close().await?;

    served?;
    dispatched??; // first `?`: JoinError, second `?`: dispatcher error
    Ok(())
}
