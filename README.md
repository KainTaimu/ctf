# Build and run
```bash
git clone https://github.com/KainTaimu/ctf.git
cd ctf
echo clubeh{FLAG} > flag.txt # Create flag file

pip install blake3 # Install prerequisite
python hash_gen.py # Create the hashed secret

cargo build --release # Build
./target/release/timing_attack # Run
```

# Running solution
```bash
python ./poc/poc.py
```
