function main() {
    let a = [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]]; // inline

    let b: [u8; (3, 2)] = [0; (3, 2)]; // initializer

    console.assert(a == b);
}