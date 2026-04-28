use crate::error::Error;
use crate::project::digest::Digests;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::project::csv::{BoopCsv, BoopRecord};

#[derive(Debug)]
pub struct BoopNode {
    name: String,
    file: File,
}

impl BoopNode {
    pub fn new(name: impl ToString, file: File) -> Self {
        Self { name: name.to_string(), file }
    }

    pub fn verify(&self) -> Result<(), Error> {
        if BufReader::new(&self.file)
            .lines()
            .next()
            .unwrap()
            .map_err(|e| Error::IO(e, self.name.clone()))?
            .replace(" ", "")
            .ne("x,y,w,h,callback,args")
        {
            return Err(Error::Custom("Boops csv header should be \"x,y,w,h,callback,args\" (spaces allowed).".to_string()))
        }
        Ok(())
    }

    pub fn as_boops(&self) -> Result<BoopCsv, Error> {
        let mut lines = BufReader::new(&self.file).lines();
        lines.next(); // Neglect first
        let mut records = vec![];
        while let Some(Ok(line)) = lines.next() {
            records.push(BoopRecord::try_from(line)?)
        }
        Ok(BoopCsv::new(&self.name, records))
    }

    pub fn digest(&self, digests: &mut Digests) -> Result<(), Error> {
        Ok(digests
            .boops_mut()
            .push(self.as_boops()?.into()))
    }
}
