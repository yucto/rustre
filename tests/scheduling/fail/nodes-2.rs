use rustre::sync;

sync! {
    #![pass(2)]

    node oui() = ()
    where
        () = non();

    node non() = ()
    where
        () = oui();
}

fn main() {}
