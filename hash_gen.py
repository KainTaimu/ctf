from blake3 import blake3  # noqa: F403

FLAG = "clubeh{71M1NG_4774CK}"

hashes = []
hasher = blake3()
for s in FLAG:
    hasher = hasher.update(bytes(s, "utf-8"))
    hash_hex = hasher.hexdigest()
    hashes.append(hash_hex)
    print(s, hash_hex)

out = """pub(crate) const SECRETL: &[&[u8]] = &["""

for hash in hashes:
    out += "\n\t" + f'b"{hash}",'

out += "\n];"

with open("./src/constants.rs", "w") as f:
    f.write(out)

print(out)
