#https://www.w3schools.com/python/python_regex.asp
import re
totalsum = 0
totalsum2 = 0
enabled = True

def part1(line):
    sum = 0
    mul = re.findall(r"mul\((\d+),(\d+)\)", line)
    for pair in mul:
        sum += int(pair[0]) * int(pair[1])
    return int(sum)

def part2(line):
    sum = 0
    global enabled
    value = re.findall(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))", line)
    for pair in value:
        if pair[0] == "don't()":
            enabled = False
        elif pair[0] == "do()":
            enabled = True
        elif enabled:
            sum += int(pair[1]) * int(pair[2])
    return int(sum)

with open("input.txt", "r") as file:
    for line in file:
        totalsum += part1(line)
        totalsum2 += part2(line)
    print(totalsum)
    print(totalsum2)