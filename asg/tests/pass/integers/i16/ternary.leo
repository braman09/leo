function main(s: bool, a: i16, b: i16, c: i16) {
    let r = if s ? a : b;

    console.assert(r == c);
}