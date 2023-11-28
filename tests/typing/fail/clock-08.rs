use rust_sync::sync;

sync! {
    node oui(c : bool) = ()
    where
        a : float = merge c {
            true => 3.0 when c,
            false => (2.0 when c) whennot c,
        };
}

fn main() {}