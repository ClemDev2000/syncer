use data_encoding::HEXLOWER;
use ring::digest::{Context, SHA256};
use std::io;
use std::io::Read;

pub fn sha256_text(payload: String) -> io::Result<String> {
    let mut context = Context::new(&SHA256);
    context.update(payload.as_bytes());
    let hash = context.finish();
    Ok(HEXLOWER.encode(hash.as_ref()))
}

pub fn sha256_file<R: Read>(mut reader: R) -> io::Result<String> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    let digest = context.finish();

    let hash = HEXLOWER.encode(digest.as_ref());

    Ok(hash)
}
