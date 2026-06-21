/// Converts confusable Unicode characters in the input to their ASCII equivalents.
///
/// Maps Cyrillic homoglyphs (р→p, а→a, о→o, е→e, с→c, у→y, х→x, і→i)
/// and the Latin f-hook (ƒ→f) to their ASCII counterparts.
///
/// Characters without a mapping are passed through unchanged.
pub fn to_ascii_equivalent(input: &str) -> String {
    input.chars().map(map_char).collect()
}

fn map_char(c: char) -> char {
    match c {
        'ƒ' => 'f',
        'р' => 'p',
        'а' => 'a',
        'о' => 'o',
        'е' => 'e',
        'с' => 'c',
        'у' => 'y',
        'х' => 'x',
        'і' => 'i',
        _ => c,
    }
}
