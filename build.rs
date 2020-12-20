mod d111 {
    include!("src/bin/111/build.rs");
}

mod d112 {
    include!("src/bin/112/build.rs");
}

mod d191 {
    include!("src/bin/191/build.rs");
}

fn main() {
    d111::prepare();
    d112::prepare();
    d191::prepare();
}
