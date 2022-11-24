use anyhow::Result;
use thrift::server::TProcessor;

use commands::{Count, CounterSyncHandler, CounterSyncProcessor};
use rpc::COMProtocol;
use thrift_import::import;

import!(commands);

fn main() -> Result<()> {
    let port = serialport::new("COM4", 115200).open_native()?;
    let processor = CounterSyncProcessor::new(CounterServer);

    let (mut i_prot, mut o_prot) = COMProtocol::pair(port) ;
    loop {
        processor.process(&mut i_prot, &mut o_prot)?;
    }
}

struct CounterServer;

impl CounterSyncHandler for CounterServer {
    fn handle_increment(&self, mut count: Count) -> thrift::Result<Count> {
        count.value = count.value.map(|v| v + 1);
        println!("Processed message: count = {:?}", count.value);
        Ok(count)
    }
}
