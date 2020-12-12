#[derive(Debug)]
pub struct FileChunkIterator {
    chunks: Vec<String>,
    c: usize,
}

impl FileChunkIterator {
    pub fn new(chunks: Vec<String>) -> Self {
        Self { chunks, c: 0 }
    }
}

impl Iterator for FileChunkIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.c == self.chunks.len() {
            return None;
        }

        let next = self.chunks[self.c].clone();

        self.c += 1;

        Some(next)
    }
}
