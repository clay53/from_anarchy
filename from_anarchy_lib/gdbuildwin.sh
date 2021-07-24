cargo build --lib --release --features godot --target x86_64-pc-windows-gnu
cp ./target/x86_64-pc-windows-gnu/release/from_anarchy_lib.dll ../from_anarchy_client/from_anarchy_lib.dll
read -p "Press enter to continue"
