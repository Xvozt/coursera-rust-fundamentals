#[derive(Debug, Copy, Clone)]

enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    VaotsDzor,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}
//here returning popularity as a &str literal because we dont need to own the string
fn popularity(w: WineRegions) -> &'static str {
    match w {
        WineRegions::Bordeaux => "Very popular",
        WineRegions::Burgundy => "Very popular",
        WineRegions::Champagne => "Quite popular",
        WineRegions::Tuscany => "Moderately popular",
        WineRegions::Rioja => "Moderately popular",
        WineRegions::NapaValley => "Popular in US",
        WineRegions::VaotsDzor => "Hidden gem",
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Areni"),
        region: WineRegions::VaotsDzor,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 1: {} from {:?}", wine3.name, wine3.region);
    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);

    let region = wine3.region;
    println!("Popularity of {:?} is {}", region, popularity(region))
}
