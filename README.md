# Build and run
```bash
git clone https://github.com/KainTaimu/ctf.git
cd ctf
echo clubeh{FLAG} > flag.txt

pip install blake3
python hash_gen.py # Create the hashed secret

cargo build --release --default-features
./target/release/timing_attack
```

# Running solution
```bash
pip install blake3
python ./poc/poc.py
```
