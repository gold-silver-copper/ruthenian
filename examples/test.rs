use ruthenian::*;

fn main() {
    /*
    let guessed_noun = ISV::decline_noun("czjelovjek", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_noun = ISV::decline_noun("djenj", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_noun = ISV::decline_noun("vjesna", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_noun = ISV::decline_noun("dom", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_noun = ISV::decline_noun("muzz", &Case::Gen, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
    let guessed_adj = ISV::decline_adj(
        "samyj",
        &Case::Gen,
        &Number::Singular,
        &Gender::Masculine,
        true,
    );
    println!("{:#?}", guessed_adj);
    let guessed_adj = ISV::decline_adj(
        "samyj",
        &Case::Gen,
        &Number::Singular,
        &Gender::Masculine,
        true,
    );
    println!("{:#?}", guessed_adj);
    let guessed_adj = ISV::decline_adj(
        "teplyj",
        &Case::Gen,
        &Number::Singular,
        &Gender::Neuter,
        true,
    );
    println!("{:#?}", guessed_adj);
    let guessed_adj = ISV::decline_adj(
        "nizkij",
        &Case::Gen,
        &Number::Singular,
        &Gender::Feminine,
        true,
    );
    println!("{:#?}", guessed_adj);
    */

    let verbiki = [
        "uczitj",
        "bratj",
        "vernutj",
        "risovatj",
        "pljevatj",
        "tancevatj",
    ];

    for verbik in verbiki {
        let guessed_verb = RUTH::conjugate_verb(
            verbik,
            &Person::First,
            &Number::Singular,
            &Gender::Feminine,
            &Tense::Present,
        );
        println!("{:#?}", guessed_verb);
        let guessed_verb = RUTH::conjugate_verb(
            verbik,
            &Person::Second,
            &Number::Singular,
            &Gender::Feminine,
            &Tense::Present,
        );
        println!("{:#?}", guessed_verb);
        let guessed_verb = RUTH::conjugate_verb(
            verbik,
            &Person::Third,
            &Number::Singular,
            &Gender::Feminine,
            &Tense::Present,
        );
        println!("{:#?}", guessed_verb);
        let guessed_verb = RUTH::conjugate_verb(
            verbik,
            &Person::First,
            &Number::Plural,
            &Gender::Feminine,
            &Tense::Present,
        );
        println!("{:#?}", guessed_verb);
        let guessed_verb = RUTH::conjugate_verb(
            verbik,
            &Person::Second,
            &Number::Plural,
            &Gender::Feminine,
            &Tense::Present,
        );
        println!("{:#?}", guessed_verb);
        let guessed_verb = RUTH::conjugate_verb(
            verbik,
            &Person::Third,
            &Number::Plural,
            &Gender::Feminine,
            &Tense::Present,
        );
        println!("{:#?}", guessed_verb);
    }

    let lik = RUTH::l_participle("buditj", &Gender::Feminine, &Number::Singular);
    println!("{:#?}", lik);

    //println!("{:#?}", ISVUTILS::string_without_last_n("daj", 2));
    let guessed_noun = RUTH::decline_noun("sluga", &Case::Ins, &Number::Singular);
    println!("{:#?}", guessed_noun.0);

    //println!("{:#?}", ISV::feminine_nouns);
    // println!("{:#?}", ISV::neuter_nouns);
    //Output: "hibiscorum"
}
