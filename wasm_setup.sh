# run when on new pc
cd ..
mkdir bloons_randomizer_web
git clone https://github.com/Warhorst/bloons_randomizer.git

# use git checkout --orphan web to create web branch if not already done

rustup target add wasm32-unknown-unknown
cargo install --locked trunk