#Looked at hyperneutrino's solution
file = open("input.txt")

rules = []
sum = 0
sum2 = 0

for line in file:
    if line.isspace(): break
    rules.append(list(map(int, line.split("|"))))

cache = {}

for x,y in rules:
    cache[(x, y)] = True
    cache[(y, x)] = False


def is_ordered(update):
    for i in range(len(update)):
        for j in range(i + 1, len(update)):
            key = (update[i], update[j])
            if key in cache and not cache[key]:
                return False
    return True

def sort(update):
    for i in range(len(update)):
        for j in range(i+1, len(update)):
            key = (update[i], update[j])
            if key in cache and not cache[key]:
                key = update[j], update[i]
# Part 1
for line in file:
    update = list(map(int, line.split(",")))
    if is_ordered(update):
        sum += update[len(update)//2]
    else:
        for i in range(len(update)):
            for j in range(i+1, len(update)):
                key = (update[i], update[j])
                if key in cache and not cache[key]:
                    update[i], update[j] = update[j], update[i]
        sum2 += update[len(update)//2]

print(sum)
print(sum2)
