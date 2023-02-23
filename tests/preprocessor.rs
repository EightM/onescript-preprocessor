use onescript_preprocessor::preprocessor::Preprocessor;

#[test]
fn test_remove_directive() {
    let example = "#region Test\nProcedure Test()\nEndProcedure\n#EndRegion";

    let expected = " \nProcedure Test()\nEndProcedure\n ";

    let preprocessor = Preprocessor::new();
    let result = preprocessor.preprocess(example);
    assert_eq!(expected, result);
}
