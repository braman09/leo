function main() {
    let a = [[[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]],
               [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]],
               [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]],
               [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]]]; // inline

    let b: [[[u8; 2]; 3]; 4] = [0; (4, 3, 2)]; // initializer

    console.assert(a == b);
}