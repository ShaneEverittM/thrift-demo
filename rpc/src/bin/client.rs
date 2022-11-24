use anyhow::Result;

use commands::{Count, CounterSyncClient, TCounterSyncClient};
use rpc::COMProtocol;
use thrift_import::import;

import!(commands);

fn main() -> Result<()> {
    let port = serialport::new("COM2", 115200).open_native()?;
    let (i_prot, o_prot) = COMProtocol::pair(port);
    let mut client = CounterSyncClient::new(i_prot, o_prot);

    let count = Count::new(0);
    let incremented = client.increment(count)?;

    dbg!(&incremented);

    let incremented = client.increment(incremented)?;

    dbg!(incremented);

    Ok(())
}
