pub trait TestCasesSchemaDocument {
    type TestCase: TestCase;
    fn tests(&self) -> Vec<Self::TestCase>;
}
pub trait TestCase {
    type Document: TestDocument;
    fn id(&self) -> &str;
    fn group(&self) -> TestGroup;
    fn failures(&self) -> Vec<Self::Document>;
    fn valid(&self) -> Option<Vec<Self::Document>>;
}
pub trait TestDocument {
    fn name(&self) -> &str;
    fn valid(&self) -> bool;
}
pub enum TestGroup {
    Mandatory,
    Recommended,
    Informative,
}
