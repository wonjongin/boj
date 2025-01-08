import sys

input = sys.stdin.readline
n = int(input())
arr = map(int, input().rstrip().split(" "))
res = []


index = 1
for e in arr:
    res.insert(len(res) - e, index)
    index += 1

print(" ".join(map(str, res)))
