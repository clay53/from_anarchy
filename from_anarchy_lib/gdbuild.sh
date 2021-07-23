cargo build --release --features godot --target x86_64-unknown-linux-gnu --target x86_64-pc-windows-gnu -Zmultitarget 
cp ./target/x86_64-unknown-linux-gnu/release/libfrom_anarchy_lib.so ../from_anarchy_client/from_anarchy_lib.so
cp ./target/x86_64-pc-windows-gnu/release/from_anarchy_lib.dll ../from_anarchy_client/from_anarchy_lib.dll