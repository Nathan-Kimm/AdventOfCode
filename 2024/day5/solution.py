with open("input.txt", "r") as file:
    rules, updates = file.read().strip().split("\n\n")
    rules_list = []
    for line in rules.split("\n"):
        a, b = line.split("|")
        rules_list.append((int(a), int(b)))
    updates = [list(map(int, line.split(","))) for line in updates.split("\n")]