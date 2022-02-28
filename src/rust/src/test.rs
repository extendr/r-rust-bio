use extendr_api::prelude::*;

#[test]
fn test_gff() {
    test! {
        //     let text = b"P0A7B8\tUniProtKB\tInitiator methionine\t1\t1\t.\t.\t.\t\
        // Note=Removed,Obsolete;ID=test
        // P0A7B8\tUniProtKB\tChain\t2\t176\t50\t+\t.\tNote=ATP-dependent protease subunit HslV;\
        // ID=PRO_0000148105";
        //     std::fs::write("test_data/test.gff", text).unwrap();
        let gff = crate::io::gff::GFF::from_file("test_data/test.gff");
        let df = gff.as_dataframe();

        assert!(df.inherits("data.frame"));
        assert_eq!(df.len(), 9);
        assert_eq!(df.dollar("seqname").unwrap(), r!(["P0A7B8", "P0A7B8"]));
    }
}
