with open ("input.txt", "r") as file:
    lines = file.readlines()

arr = [list(line.strip()) for line in lines]


def part1():
    count = 0
    for i in range(len(arr)):
        for j in range(len(arr[0])):
            if j + 3 < len(arr[0]):
                #Checks Forward
                if (arr[i][j] + arr[i][j+1] + arr[i][j+2] + arr[i][j+3]) == "XMAS" or (arr[i][j] + arr[i][j+1] + arr[i][j+2] + arr[i][j+3]) == "SAMX":
                    count += 1
            if i + 3 < len(arr) and j + 3 < len(arr[0]):
                #Checks Diagonally
                if (arr[i][j] + arr[i+1][j+1] + arr[i+2][j+2] + arr[i+3][j+3]) == "XMAS" or (arr[i][j] + arr[i+1][j+1] + arr[i+2][j+2] + arr[i+3][j+3]) == "SAMX":
                    count += 1
            if i + 3 < len(arr):
                #Checks Vertically
                if (arr[i][j] + arr[i+1][j] + arr[i+2][j] + arr[i+3][j]) == "XMAS" or (arr[i][j] + arr[i+1][j] + arr[i+2][j] + arr[i+3][j]) == "SAMX":
                    count += 1
            if i + 3 < len(arr) and j > 2:
                #Checks Diagonally Other Way
                if (arr[i][j] + arr[i+1][j-1] + arr[i+2][j-2] + arr[i+3][j-3]) == "XMAS" or (arr[i][j] + arr[i+1][j-1] + arr[i+2][j-2] + arr[i+3][j-3]) == "SAMX":
                    count += 1
    print(count)


def part2():
    count = 0
    str1 = ""
    str2 = ""
    for i in range(len(arr) - 2):
        for j in range(len(arr[0]) - 2):
            str1 = arr[i][j] + arr[i+1][j+1] + arr[i+2][j+2]
            str2 = arr[i][j+2] + arr[i+1][j+1] + arr[i+2][j]
            if (str1 == "MAS" or str1 == "SAM") and (str2 == "MAS" or str2 == "SAM"):
                count += 1
    print(count)


part1()
part2()