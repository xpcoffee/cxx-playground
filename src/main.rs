// General notes:
// always do clean build - the cxx build pipeline doesnt surface all cpp compilation errors

fn main() {
    let client = ffi::new_blobstore_client();

    let chunks = vec![b"fearless".to_vec(), b"concurrent".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.put(&mut buf);
    println!("blobid = {}", blobid);
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        // Question - How to have methods on types in rust?
        // impl Multibuf { fn ... } <--- ?
        // fn (&self: Multibuf) {...}     <--- ?
        type MultiBuf;

        fn to_string(&self) -> String;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    unsafe extern "C++" {
        include!("cxx-playground/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;

        // Question - wonder what happens if we give a non mutable reference?
        fn put(&self, parts: &mut MultiBuf) -> u64;
    }
}

pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}

impl MultiBuf {
    fn to_string(&self) -> String {
        self.pos.to_string() + " | " + self.chunks.len().to_string().as_str()
    }
}

pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}
