
enum ExpansionTypes {
    GeneticApex,
    PromoA,
}

enum ExpansionPackTypes {
    Glurak,
    Mewtu,
    Pikachu,
}

enum ExpansionCodes {
    A1
}

struct PokeIndex {
    expansion: ExpansionTypes, //i.e. Genetic Apex
    expansion_code: ExpansionCodes, // i.e. for Genetic Apex: A1
    number: Int,
}

enum EnergyTypes {

    //For Pokemon
    Colorless,
    Darkness,
    Dragon,
    Fairy,
    Fighting,
    Fire,
    Grass,
    Lightning,
    Metal,
    Psychic,
    Water,

    //For Trainer Cards:
    None,
}

enum RarityTypes {
    //This might be overbuilt and instead a simple Int value corresponding to rarity types could be used (1-8 for example)
    //Diamond Rarity
    D1,
    D2,
    D3,
    D4,

    //Stars
    S1,
    S2,
    S3,

    //Crown
    C,


}

struct Packs {
    expansion: ExpansionTypes,
    name: &str, // i.e. for Genetic Apex: Glurak
    // TODO: Table for all cards obtainable specific to each pack
}

struct Pokemon {
    index: PokeIndex, //should be unique to every card (not withstanding language)
    name: &str,
    energy: EnergyTypes,
    rarity: RarityTypes,
    packs: [Packs], //for multiple packs a list?
    shop: bool, // true if buyable in shop

}

static POKEMONLIST: [Pokemon] = vec![
    Pokemon {
        index: PokeIndex { expansion: ExpansionTypes::GeneticApex, expansion_code: ExpansionCodes::A1, number: 1 },
        name: "Bulbasaur",
        energy: EnergyTypes::Grass,
        rarity: RarityTypes::D1,
        packs: [Packs { expansion: ExpansionTypes::GeneticApex, name: ExpansionPackTypes::Mewtu }],
        shop: false,

    },

    Pokemon {
        index: PokeIndex { expansion: ExpansionTypes::GeneticApex, expansion_code: ExpansionCodes::A1, number: 2 },
        name: "Ivysaur",
        energy: EnergyTypes::Grass,
        rarity: RarityTypes::D2,
        packs: [Packs { expansion: ExpansionTypes::GeneticApex, name: ExpansionPackTypes::Mewtu }],
        shop: false,

    },

    Pokemon {
        index: PokeIndex { expansion: ExpansionTypes::GeneticApex, expansion_code: ExpansionCodes::A1, number: 3 },
        name: "Venusaur",
        energy: EnergyTypes::Grass,
        rarity: RarityTypes::D3,
        packs: [Packs { expansion: ExpansionTypes::GeneticApex, name: ExpansionPackTypes::Mewtu }],
        shop: false,

    },

    Pokemon {
        index: PokeIndex { expansion: ExpansionTypes::GeneticApex, expansion_code: ExpansionCodes::A1, number: 4 },
        name: "Venusaur-ex",
        energy: EnergyTypes::Grass,
        rarity: RarityTypes::D1,
        packs: [Packs { expansion: ExpansionTypes::GeneticApex, name: ExpansionPackTypes::Mewtu }],
        shop: false,

    },

    Pokemon {
        index: PokeIndex { expansion: ExpansionTypes::GeneticApex, expansion_code: ExpansionCodes::A1, number: 5 },
        name: "Caterpie",
        energy: EnergyTypes::Grass,
        rarity: RarityTypes::D1,
        packs: [Packs { expansion: ExpansionTypes::GeneticApex, name: ExpansionPackTypes::Pikachu }],
        shop: false,

    },

    Pokemon {
        index: PokeIndex { expansion: ExpansionTypes::GeneticApex, expansion_code: ExpansionCodes::A1, number: 6 },
        name: "Metapod",
        energy: EnergyTypes::Grass,
        rarity: RarityTypes::D1,
        packs: [Packs { expansion: ExpansionTypes::GeneticApex, name: ExpansionPackTypes::Pikachu }],
        shop: false,

    },

    Pokemon {
        index: PokeIndex { expansion: ExpansionTypes::GeneticApex, expansion_code: ExpansionCodes::A1, number: 7 },
        name: "Butterfree",
        energy: EnergyTypes::Grass,
        rarity: RarityTypes::D3,
        packs: [Packs { expansion: ExpansionTypes::GeneticApex, name: ExpansionPackTypes::Pikachu }],
        shop: false,

    },
];



fn main() {
    println!("Hello, world!");
}
