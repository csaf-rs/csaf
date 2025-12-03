use crate::test_structure::{TestCase, TestCasesSchemaDocument, TestDocument, TestGroup};

impl TestCasesSchemaDocument for crate::schema::csaf2_1::testcases_schema::TestCasesForCsaf {
    type TestCase = crate::schema::csaf2_1::testcases_schema::TestT;
    fn tests(&self) -> Vec<Self::TestCase> {
        self.tests.clone()
    }
}

impl TestCase for crate::schema::csaf2_1::testcases_schema::TestT {
    type Document = crate::schema::csaf2_1::testcases_schema::FileT;
    fn id(&self) -> &str {
        &self.id
    }
    fn group(&self) -> TestGroup {
        match self.group {
            crate::schema::csaf2_1::testcases_schema::TestGroup::Mandatory => TestGroup::Mandatory,
            crate::schema::csaf2_1::testcases_schema::TestGroup::Recommended => TestGroup::Recommended,
            crate::schema::csaf2_1::testcases_schema::TestGroup::Informative => TestGroup::Informative,
        }
    }
    fn failures(&self) -> Vec<Self::Document> {
        self.failures.clone()
    }
    fn valid(&self) -> Option<Vec<Self::Document>> {
        self.valid.clone()
    }
}

impl TestDocument for crate::schema::csaf2_1::testcases_schema::FileT {
    fn name(&self) -> &str {
        &self.name
    }
    fn valid(&self) -> bool {
        self.valid
    }
}
