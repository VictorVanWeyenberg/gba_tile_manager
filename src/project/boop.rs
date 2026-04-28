use crate::error::Error;
use crate::project::csv::{BoopCsv, BoopRecord};
use crate::project::digest::Digests;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

#[derive(Debug)]
pub struct BoopNode {
    name: String,
    file: File,
}

impl BoopNode {
    pub fn new(name: impl ToString, file: File) -> Self {
        Self {
            name: name.to_string(),
            file,
        }
    }

    pub fn verify(&mut self) -> Result<(), Error> {
        if BufReader::new(&self.file)
            .lines()
            .next()
            .unwrap()
            .map_err(|e| Error::IO(e, self.name.clone()))?
            .replace(" ", "")
            .ne("x,y,w,h,callback,args")
        {
            return Err(Error::Custom(
                "Boops csv header should be \"x,y,w,h,callback,args\" (spaces allowed)."
                    .to_string(),
            ));
        }
        self.file.seek(SeekFrom::Start(0))
            .map_err(|e| Error::IO(e, format!("Could not seek in {:?}", &self.file)))?;
        Ok(())
    }

    pub fn as_boops(&self) -> Result<BoopCsv, Error> {
        let records = BufReader::new(&self.file)
            .lines()
            .skip(1)
            .map(|line| {
                line.map_err(|e| Error::IO(e, format!("Could not read line from {:?}", &self.file)))
                    .and_then(BoopRecord::try_from)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(BoopCsv::new(self.name.clone(), records))
    }

    pub fn digest(&self, digests: &mut Digests) -> Result<(), Error> {
        digests.boops_mut().push(self.as_boops()?.into());
        Ok(())
    }
}
