import re

while True:
        try:
            line = input()
            matches = re.findall(r"[0-9]*, (\w*)\(",line)
            if matches:
                print(matches[0])
        except EOFError:
            # no more information
            break

