function main () -> i8 {
    if true {
        return 1i8   //ignored
    }
    return 2i8       //ignored
    return 3i8       //returns
}