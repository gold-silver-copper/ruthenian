#[derive(Debug, PartialEq, Clone)]
pub struct VerbEndings {
    pub first_singular: &'static str,
    pub second_singular: &'static str,
    pub third_singular: &'static str,
    pub first_plural: &'static str,
    pub second_plural: &'static str,
    pub third_plural: &'static str,
}

pub const FIRST_CONJUGATION: VerbEndings = VerbEndings {
    first_singular: "u",
    second_singular: "esz",
    third_singular: "et",
    first_plural: "em",
    second_plural: "ete",
    third_plural: "ut",
};

pub const SECOND_CONJUGATION: VerbEndings = VerbEndings {
    first_singular: "ju",
    second_singular: "isz",
    third_singular: "et",
    first_plural: "im",
    second_plural: "ite",
    third_plural: "ut",
};
