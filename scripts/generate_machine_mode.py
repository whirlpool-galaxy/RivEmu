import csv

rows = []

with open("Mcsr.csv", "r") as csv_file:
    
    csv_reader = csv.reader(csv_file, delimiter=";")

    for row in csv_reader:
        rows.append(row)

for row in rows:
    print("struct {} {{".format(row[2][0].upper() + row[2][1:]))
    print("// TODO")
    print("}")
    print()

for row in rows:
    print("impl {} {{".format(row[2][0].upper() + row[2][1:]))
    print("// TODO")
    print("}")
    print()
    print("impl ZicsrRegister for {} {{".format(row[2][0].upper() + row[2][1:]))
    print("fn get_access_mode() -> AccessMode {")
    print("AccessMode::{}".format(row[1]))
    print("}")
    print("fn get_zicsr() -> ZiscrVal {")
    print("// TODO")
    print("ZiscrVal::Mxlen32(0)")
    print("}")
    print()
    print("fn set_zicsr(value: ZiscrVal) -> bool {")
    print("// TODO")
    print("false")
    print("}")
    print("}")
    print()