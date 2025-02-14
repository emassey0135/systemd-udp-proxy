use std::{io, sync::Arc, time::Duration};

use tokio::{net::UdpSocket, sync::mpsc::UnboundedReceiver, time::timeout};

use log::info;

use crate::{
    error_util::{handle_io_error, ErrorAction},
    session::SessionReply,
};

/// Loops infinitely over the `reply_channel_rx` to forward traffic from the destination of the proxy.
///
/// This task recives channel messages representing responses from the proxy destination over
/// `reply_channel_tx` from [crate::session::Session]s and sends them back to the original
/// source via `tx_socket`.
pub async fn tx_task(
    idle_timeout: u64,
    mut reply_channel_rx: UnboundedReceiver<SessionReply>,
    tx_socket: Arc<UdpSocket>,
) -> io::Result<()> {
    let duration = Duration::from_secs(idle_timeout);
    loop {
        let result = if idle_timeout > 0 { timeout(duration, reply_channel_rx.recv()).await } else { Ok(reply_channel_rx.recv().await) };
        match result {
            Ok(None) => {
                return Ok(());
            },
            Ok(Some(reply)) => {
                match tx_socket
                    .send_to(&reply.data, (reply.source.address, reply.source.port))
                    .await
                {
                    Ok(_) => continue,
                    Err(err) => match handle_io_error(err) {
                        ErrorAction::Terminate(err) => return Err::<(), io::Error>(err),
                        ErrorAction::Continue => continue,
                    },
                }
            },
            Err(_timeout_exceeded) => {
                info!("Closing tx packet handler");
                return Ok(());
            },
        }
    }
}
