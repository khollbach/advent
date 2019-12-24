from typing import List
import fileinput, math

Image = List[List[List[int]]]

def read_image(img_str: str, HEIGHT: int, WIDTH: int, DEPTH: int) -> Image:
    assert DEPTH * HEIGHT * WIDTH == len(img_str)  # Divides evenly.
    img = []
    for i in range(DEPTH):
        img.append([])  # layer i
        for j in range(HEIGHT):
            img[i].append([])  # row j
            for k in range(WIDTH):
                digit = int(img_str[i*HEIGHT*WIDTH + j*WIDTH + k])
                img[i][j].append(digit)  # column k
    return img

def count_occurances(img: Image, layer: int, digit: int) -> int:
    count = 0
    i = layer
    for j in range(len(img[i])):
        for k in range(len(img[i][j])):
            if img[i][j][k] == digit:
                count += 1
    return count

def checksum(img: Image) -> int:
    D = len(img)
    count, layer = min((count_occurances(img, i, 0), i) for i in range(D))
    return count_occurances(img, layer, 1) * count_occurances(img, layer, 2)

def visible_digit(img: Image, row: int, col: int) -> int:
    j, k = row, col
    for i in range(len(img)):
        digit = img[i][j][k]
        if digit != 2:
            return digit
    return 2

def render(img: Image) -> List[List[int]]:
    D, H, W = len(img), len(img[0]), len(img[0][0])

    rendered = []
    for j in range(H):
        rendered.append([])  # row j
        for k in range(W):
            rendered[j].append(visible_digit(img, j, k))  # col k

    return rendered

def display_pixel(digit: int) -> str:
    if digit == 1:  # White
        return "#"
    elif digit == 0:  # Black
        return " "
    else:
        assert digit == 2  # Transparent
        return "."

def display_img(img: Image) -> str:
    rendered = render(img)
    return "".join("".join(map(display_pixel, row)) + "\n" for row in rendered)

if __name__ == "__main__":
    img_str = fileinput.input().readline().rstrip("\n")

    HEIGHT = 6
    WIDTH = 25
    DEPTH = len(img_str) // (HEIGHT * WIDTH)

    img = read_image(img_str, HEIGHT, WIDTH, DEPTH)

    print(checksum(img))
    print(display_img(img), end="")
