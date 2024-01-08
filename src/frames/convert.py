#!/usr/bin/python
import icon

def convert_icon(input_file: str, output_file: str):
    out_file = icon.file2image(input_file)
    out_file.write(output_file)



if __name__ == "__main__":
    for i in range(0, 55):
        convert_icon("%04d.png" % i, "%04d.icon" % i)