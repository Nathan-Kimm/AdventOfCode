left = []
right = []

f = open("input.txt", "r")
data = f.readline()

while data != "":
    data = data.strip("\n")
    dataLine = data.split("   ")
    left.append(dataLine[0])
    right.append(dataLine[1])
    data=f.readline()

left.sort()
right.sort()

#Problem 1
difference = 0
for i in range(len(left)):
    difference += abs(int(left[i]) - int(right[i]))
print(difference)

#Problem 2
similarity = 0
for i in range(len(left)):
    similarity += int(left[i]) * right.count(left[i])
print(similarity)