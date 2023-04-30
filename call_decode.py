import sys
import time
from decode_and_rate import decode

base64_string = sys.argv[1]
jsprogram = sys.argv[2]

out = decode(base64_string, jsprogram)
while len(out) == 0:
    pass

