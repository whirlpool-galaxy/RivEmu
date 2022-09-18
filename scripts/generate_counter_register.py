# Copyright (C) 2022 Jonathan Schild - MIT License

start = 0x3A0
for i in range(0, 15):
    print("0x{:3X};MRW;pmpcfg{}".format(start + i, i))
print("=============================")

start = 0x3B0
for i in range(0, 63):
    print("0x{:3X};MRW;pmpaddr{}".format(start + i, i))
print("=============================")

start = 0xB03
for i in range(3, 31):
    print("0x{:3X};MRW;mhpmcounter{}".format(start + i -3, i))
print("=============================")

start = 0xB83
for i in range(3, 31):
    print("0x{:3X};MRW;mhpmcounter{}h".format(start + i -3, i))
print("=============================")

start = 0x323
for i in range(3, 31):
    print("0x{:3X};MRW;mhpmevent{}".format(start + i -3, i))