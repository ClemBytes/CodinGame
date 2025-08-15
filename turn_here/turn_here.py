input = input().split(" ")

direction = input[0]
nb_arrows = int(input[1])
height = int(input[2])
thickness = int(input[3])
spacing = int(input[4])
indent = int(input[5])

if direction == "left":
    char = '<'
    current_indent = indent * ((height - 1) // 2)
elif direction == "right":
    char = '>'
    current_indent = 0
else:
    raise(f"Unkwnow direction: {direction}")


for i in range(height):
    line = current_indent * ' '
    for j in range(nb_arrows):
        line += thickness * char
        if j != nb_arrows - 1:
            line += spacing * ' '
    print(line)
    if i < (height - 1) // 2:
        if direction == "right":
            current_indent += indent
        else:
            current_indent -= indent
    else:
        if direction == "right":
            current_indent -= indent
        else:
            current_indent += indent