use rust_sync::sync;

sync! {
    #![pass(0)]
    
    node oui() = ()
    where
        x : bool = 3 >= -2;
}

fn main() {}
