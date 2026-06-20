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
