def part1():
    file = open("part1.txt", "r")
    sum = 0
    for line in file.readlines():
        left = 0
        right = len(line) - 1

        while True:
            if line[left].isdigit():
                break
            left += 1

        while True:
            if line[right].isdigit():
                break
            right -= 1

        sum += int(f'{line[left]}{line[right]}')
    
    print(f'{sum}')

    file.close()

def part2():
    words = {"one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9}
    file = open("part1.txt", "r")
    sum = 0
    for line in file.readlines():
        left = 0
        right = len(line) - 1

        lnum = None
        rnum = None

        while True:
            ltext = line[:left]
            for word in words:
                if word in ltext:
                    lnum = words.get(word)
                    break
            if lnum:
                break
            if line[left].isdigit():
                lnum = line[left]
                break

            left += 1
        while True:
            rtext = line[right:]
            for word in words:
                if word in rtext:
                    rnum = words.get(word)
                    break    
            if rnum:
                break
            if line[right].isdigit():
                rnum = line[right]
                break
            
            right -= 1

        sum += int(f'{lnum}{rnum}')

    print(sum)
    file.close()

if __name__ == "__main__":
    part2()
