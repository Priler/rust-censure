use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static FOUL_CORE: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("arse", "^arse((hole)|$)"),
        ("ass", "^ass((butt)|(idiot)|(hat)|(jabber)|(pirate)|(bag)|(banger)|(bandit)|(bite)|(clown)|(cock)|(cracker)|(es)|(face)|(goblin)|(hat)|(head)|(hole)|(hopper)|(jacker)|(lick)|(licker)|(monkey)|(munch)|(nigger)|(hit)|(sucker)|(ucker)|(wad)|(wipe)|$)"),
        ("bitch", "^bitch"),
        ("bullshit", "^bullshit$"),
        ("butt", "^butt((plug)|(pirate)|($))"),
        ("clit", "^clit(($)|(or)|(face))"),
        ("cum", "^cum(($)|(bubble)|(dumpster)|(guzzler)|(jockey)|(slut)|(tart))"),
        ("cunni", "^cunni(($)|(e)|(lingus))"),
        ("cock", "^cock($|(ass)|(bite)|(burger)|(face)|(head)|(jockey)|(knoker)|(master)|(mong(ler|ruel))|(monkey)|(muncher)|(nose)|(nugget)|(shit)|(smith)|(smoke)|(sniffer)|(sucker)|(waffle))"),
        ("cunt", "^cunt(($)|(ass)|(face)|(hole)|(licker)|(rag)|(slut))$"),
        ("dick", "^dick(([s]*$)|(bag)|(beaters)|(face)|(head)|(hole)|(juice)|(milk)|(monger)|(slap)|(suck(er|in))|(tickler)|(wad)|(weasel)|(weed)|(wod))"),
        ("dumb", "dum(b)*($|(ass)|(shit))"),
        ("fag", "fag($|(bag)|(g[io]t)|(tard)|(ass))"),
        ("fuck", "fuck"),
        ("gay", "gay((ass)|(bob)|(do)|(lord)|(tard)|(wad))"),
        ("jackass", "jackass"),
        ("jerk", "jerk((o[f]+)|(ass))"),
        ("mothafucka", "m[oa](th|z)afuck(a|in[g]*|er)"),
        ("penis", "^penis(banger|puffer)"),
        ("pecker", "pecker(head)*"),
        ("piss", "^piss((ed)*(off)*|flaps)"),
        ("poon", "^p(oo|u)n(an(n)*[iy]|tang|$)"),
        ("prick", "^prick$"),
        ("pussy", "^puss((y)*(lick)*|ies)"),
        ("quee", "quee(f|r($|bait|hole))"),
        ("suck", "^suck(ass|$)"),
        ("shit", "^shit($|ass|bag|brains|breath|canned|cunt|dick|face|faced|head|hole|house|spitter|stain|(t)*(er|iest|ing|y))"),
        ("slut", "^slut($|bag)"),
        ("shiz", "^shiz(nit)*$"),
        ("twat", "^twat(lips|s|waffle|$)"),
        ("vjay", "^vjayjay"),
        ("wank", "^wank(job|$)"),
        ("whore", "^whore(bag|face|$)")
    ])
});

pub static FOUL_DATA: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    HashMap::from([])
});

pub static EXCLUDES_CORE: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([])
});

pub static EXCLUDES_DATA: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    HashMap::from([])
});

pub static BAD_SEMI_PHRASES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "suckmydick",
        "sickmyduck",
        "cameltoe",
    ]
});

pub static BAD_PHRASES: Lazy<Vec<&'static str>> = Lazy::new(|| vec![
    "camel(\\s)*toe",
    "dick[\\-\\s]*sneeze",
    "blow[\\-\\s]*job",
    "jerk[\\-\\s]*off",
    "nut[\\-\\s]*sack"
]);

pub static TRANS_TAB: Lazy<HashMap<char, char>> = Lazy::new(|| {
    HashMap::from([])
});
