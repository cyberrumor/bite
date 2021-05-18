#!/usr/bin/env python3

def get_trees(thelist: list):
    result = []
    for x in range(0, len(thelist)):
        y = 2
        while y <= len(thelist):
            vec = thelist[x:y]
            if len(vec) > 1:
                result.append(vec)
            y += 1
    return result
            








if __name__ == '__main__':
    thelist = [
        "Mary",
        "had",
        "a",
        "little",
        "lamb",
        "whose",
        "fleece",
        "was",
        "white",
        "as",
        "snow"
    ]

    for i in get_trees(thelist):
        print(i)
