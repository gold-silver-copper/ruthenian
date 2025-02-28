#[derive(Debug, Clone, Default)]
pub struct RUTH {}

#[derive(Debug, PartialEq, Clone)]
pub struct ComplexNoun {
    pub head_noun: String,
    pub adjective: Vec<String>,
}

impl Default for ComplexNoun {
    fn default() -> Self {
        Self {
            head_noun: "exemplum".into(),

            adjective: Vec::new(),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Singular,
    Plural,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Case {
    Nom,
    Acc,
    Gen,
    Loc,
    Dat,
    Ins,
    //vocative will be handle seperately
}
#[derive(Debug, PartialEq, Clone)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Person {
    First,
    Second,
    Third,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Conjugation {
    First,
    Second,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Tense {
    Present,
    Imperfect,
    Future,
    Perfect,
    PluPerfect,
    Conditional,
}
pub type Noun = (String, Gender);
pub type Adjective = String;
pub type Verb = String;
