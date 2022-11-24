use std::io::{Read, Write};

use serialport::COMPort;
use thrift::{
    protocol::{TBinaryInputProtocol, TBinaryOutputProtocol},
    transport::{TFramedReadTransport, TFramedWriteTransport},
};

pub struct COMProtocol(COMPort);

impl Write for COMProtocol {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Read for COMProtocol {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl COMProtocol {
    pub fn pair(
        port: COMPort,
    ) -> (
        TBinaryInputProtocol<TFramedReadTransport<COMProtocol>>,
        TBinaryOutputProtocol<TFramedWriteTransport<COMProtocol>>,
    ) {
        let i_prot = TBinaryInputProtocol::new(
            TFramedReadTransport::new(Self(
                port.try_clone_native().expect("Failed to duplicate handle"),
            )),
            false,
        );
        let o_prot = TBinaryOutputProtocol::new(TFramedWriteTransport::new(Self(port)), false);

        (i_prot, o_prot)
    }
}
