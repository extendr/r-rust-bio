use extendr_api::prelude::*;

#[test]
fn test_gff() {
    test! {
        //     let text = b"P0A7B8\tUniProtKB\tInitiator methionine\t1\t1\t.\t.\t.\t\
        // Note=Removed,Obsolete;ID=test
        // P0A7B8\tUniProtKB\tChain\t2\t176\t50\t+\t.\tNote=ATP-dependent protease subunit HslV;\
        // ID=PRO_0000148105";
        //     std::fs::write("test_data/test.gff", text).unwrap();
        let gff = GFF::from_file("test_data/test.gff");
        let df = gff.as_dataframe();
        let expected = list!(seqname=["P0A7B8", "P0A7B8"], source=["UniProtKB", "UniProtKB"], feature_type=["Initiator methionine", "Chain"], start=[1_i32, 2], end=[1_i32, 176], score=[i32::MIN, 50], frame=[".", "."]);
        assert_eq!(df, expected);
    }
}
