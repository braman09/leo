circuit Bar {
    b2: u32

    function add_five(z:u32) -> u32 {
        return z+5u32
    }
}

function main () {
    let Bar = 66u32;
    let k1 = Bar{ b2: 30u32 };
    let k2 = Bar::add_five(55u32);
}