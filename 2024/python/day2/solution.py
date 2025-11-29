f = open("input.txt", "r")
data = f.readline()
count = 0
count2 = 0

def isSafe(data):
    inc = True
    dec = True
    for i in range(len(data)-1):
        if int(data[i]) - int(data[i+1]) not in (1,2,3): 
            dec = False
        if int(data[i+1]) - int(data[i]) not in (1,2,3):
            inc = False
    if inc or dec:
        return True
    return False

#Problem 1 and 2
while data != "":
    data = data.strip("\n")
    dataSplit = data.split(" ")
    if isSafe(dataSplit):
        count+=1
    else:    
        for i in range(len(dataSplit)):
            newArray = dataSplit[:i] + dataSplit[i+1:]
            if isSafe(newArray):
                count2 += 1
                break
    data = f.readline()
print(count)
print(count2+count)
    



