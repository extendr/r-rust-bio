use crate::error::Error;
use bio::io::gff;
use multimap::MultiMap;
use bio_types::strand::Strand;

// TODO: fix the macros so this is not necessary.
use extendr_api::prelude::*;

pub struct GFF {
    data: Vec<gff::Record>,
}

fn do_from_file(path: &str) -> std::result::Result<GFF, Error> {
    let file = std::fs::File::open(path)
        .map_err(|_| Error(format!("unable to open GFF file {}", path)))?;

    let mut reader = gff::Reader::new(file, gff::GffType::GFF3);

    let data: std::result::Result<Vec<_>, _> = reader.records().collect();
    let data = data.map_err(|_| Error(format!("unable to read GFF file {}", path)))?;

    Ok(GFF { data })
}

fn attributes_to_str(map: &MultiMap<String, String>) -> String {
    let a = map.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>();
    a.join(";")
}

#[extendr]
impl GFF {
    pub fn from_file(path: &str) -> Self {
        match do_from_file(path) {
            Ok(ok) => ok,
            Err(err) => throw_r_error(err.to_string()),
        }
    }

    pub fn as_dataframe(&self) -> List {
        let seqname = Strings::from_values(self.data.iter().map(|rec| rec.seqname()));
        let source = Strings::from_values(self.data.iter().map(|rec| rec.source()));
        let feature_type = Strings::from_values(self.data.iter().map(|rec| rec.feature_type()));
        let start = Integers::from_values(self.data.iter().map(|rec| rec.start()));
        let end = Integers::from_values(self.data.iter().map(|rec| rec.end()));
        let score = Integers::from_values(self.data.iter().map(|rec| rec.score()));
        let strand = Strings::from_values(self.data.iter().map(|rec| {
            match rec.strand() {
                Some(Strand::Forward) => "+",
                Some(Strand::Reverse) => "-",
                _ => ".",
            }
        }));
        let frame = Strings::from_values(self.data.iter().map(|rec| rec.frame()));
        let attributes = Strings::from_values(self.data.iter().map(
            |rec| attributes_to_str(rec.attributes())));

        let res = extendr_api::data_frame!(
            seqname = seqname,
            source = source,
            feature_type = feature_type,
            start = start,
            end = end,
            score = score,
            strand = strand,
            frame = frame,
            attributes = attributes
        );
        res.try_into().unwrap()
    }
}

extendr_module! {
    mod gff;
    impl GFF;
}
