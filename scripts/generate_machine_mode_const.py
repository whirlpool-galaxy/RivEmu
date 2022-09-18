# Copyright (C) 2022 Jonathan Schild - MIT License

import csv

rows = []

with open("Mcsr.csv", "r") as csv_file:
    csv_reader = csv.reader(csv_file, delimiter=";")

    for row in csv_reader:
        rows.append(row)

for row in rows:
    print("const {}: u16 = {};".format(row[2].upper(), row[0]))