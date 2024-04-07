from ast import literal_eval
from itertools import batched

i = input("Enter string: ")
string = literal_eval("'" + i + "'")
hex = ["".join(x) for x in batched(string.encode('ascii').hex().upper(), 2)]
hex_groups = batched(hex, 4)

print("In groups of reversed 8s:")
for group in hex_groups:
    print("0x" + "".join(reversed(group)))
