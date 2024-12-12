with open("test.txt", "r") as file:
    lines = file.readlines()

ogarr = [list(line.strip()) for line in lines]

#Find location of Guard
for i in range(len(ogarr)):
    for j in range(len(ogarr[0])):
        if ogarr[i][j] == "^":
            xog, yog = i, j


def runGuard(x, y, arr):
    arr = ogarr
    print(''.join(arr))
    iterationCount = 0
    ingrid = True
    count = 1
    dir = 0
    while ingrid:
        #Guard Up
        if dir == 0:
            while(arr[x][y] != "#"):
                arr[x][y] = "X"
                x -= 1
                if arr[x][y] == "#":
                    dir = 1
                    x += 1
                    break
                elif arr[x][y] != "X":
                    count += 1
                if x < 0:
                    ingrid = False
                    break

                
        #Guard Right
        elif dir == 1:
            while(arr[x][y] != "#"):
                arr[x][y] = "X"
                y += 1
                if arr[x][y] == "#":
                    dir = 2
                    y -= 1
                    break
                elif arr[x][y] != "X": 
                    count += 1
                if y > len(arr[0])-2:
                    ingrid = False
                    break

                
        #Guard Down
        elif dir == 2:
            while(arr[x][y] != "#"):
                arr[x][y] = "X"
                x += 1
                if arr[x][y] == "#":
                    dir = -1
                    x -= 1
                    break
                elif arr[x][y] != "X": 
                    count += 1
                if x > len(arr)-2:
                    ingrid = False
                    break

                
        #Guard Left     
        elif dir == -1:
            while(arr[x][y] != "#"):
                arr[x][y] = "X"
                y -= 1
                if arr[x][y] == "#":
                    dir = 0
                    y += 1
                    break
                elif arr[x][y] != "X": 
                    count += 1 
                if y < 0:
                    ingrid = False 
                    break
        iterationCount += 1
        if iterationCount > 1000000:
            return True 
    print(count)
    return False

#Part 1
runGuard(xog, yog, ogarr)


# Part 2 
obstructionCount = 0

for i in range(len(ogarr)):
    for j in range(len(ogarr[0])):
        value = ogarr[i][j]
        ogarr[i][j] = "#"
        if runGuard(xog, yog, ogarr) and ogarr[i][j] != "^":
            obstructionCount += 1
        ogarr[i][j] = value
print(obstructionCount)