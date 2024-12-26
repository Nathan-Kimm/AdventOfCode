with open("input.txt", "r") as file:
    lines = file.readlines()

arr = [list(line.strip()) for line in lines]

#Find location of Guard
for i in range(len(arr)):
    for j in range(len(arr[0])):
        if arr[i][j] == "^":
            x, y = i, j

obstructionseen = set()
def runGuard(x, y, arr):
    seen = set()
    iterationCount = 0
    ingrid = True
    count = 1
    dir = 0
    while ingrid:
        #Guard Up
        if dir == 0:
            while(arr[x][y] != "#"):
                seen.add((x,y))
                x -= 1
                if arr[x][y] == "#":
                    dir = 1
                    x += 1
                    break
                elif (x,y) not in seen:
                    count += 1
                if x < 0:
                    ingrid = False
                    break

                
        #Guard Right
        elif dir == 1:
            while(arr[x][y] != "#"):
                seen.add((x,y))
                y += 1
                if arr[x][y] == "#":
                    dir = 2
                    y -= 1
                    break
                elif (x,y) not in seen: 
                    count += 1
                if y > len(arr[0])-2:
                    ingrid = False
                    break

                
        #Guard Down
        elif dir == 2:
            while(arr[x][y] != "#"):
                seen.add((x,y))
                x += 1
                if arr[x][y] == "#":
                    dir = -1
                    x -= 1

                    break
                elif (x,y) not in seen: 
                    count += 1
                if x > len(arr)-2:
                    ingrid = False
                    break

                
        #Guard Left     
        elif dir == -1:
            while(arr[x][y] != "#"):
                seen.add((x,y))
                y -= 1
                if arr[x][y] == "#":
                    dir = 0
                    y += 1
                    break
                elif (x,y) not in seen: 
                    count += 1 
                if y < 0:
                    ingrid = False 
                    break
        iterationCount += 1
        if iterationCount > 10000:
            obstructionseen.update(seen)
            return True 
    print(count)
    return False

#Part 1
runGuard(x, y, arr)


# Part 2 
obstructionCount = 0
for i in range(len(arr)):
    for j in range(len(arr[0])):
        value = arr[i][j]
        if(i,j) not in obstructionseen:
            arr[i][j] = "#"
        if runGuard(x, y, arr) and arr[i][j] != "^":
            obstructionCount += 1
        arr[i][j] = value
print(obstructionCount)