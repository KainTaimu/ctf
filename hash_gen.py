from blake3 import blake3  # noqa: F403

with open("./flag.txt", "r") as f:
    flag = f.readline().strip()

hashes = []
hasher = blake3()
for s in flag:
    byte = bytes(s, "utf-8")
    hasher = hasher.update(byte)
    hash_hex = hasher.hexdigest()
    hashes.append(hash_hex)
    print(s, int.from_bytes(byte), hash_hex)

out = """pub(crate) const SECRETL: &[&[u8]] = &["""

n = 0
for hash in hashes:
    out += "\n\t" + f'b"{hash}",'
    n += 1

print("len:", n)

out += "\n];"

with open("./src/constants.rs", "w") as f:
    f.write(out)

print(out)
