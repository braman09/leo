function array_3x2_nested() -> [u8; (3, 2)] {
    return [[0u8; 3]; 2] // The correct 3x2 nested array is `[0u8; 2]; 3]`
}

function main() {
    let a = array_3x2_nested();
}