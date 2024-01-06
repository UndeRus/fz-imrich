#!/bin/bash -e
for FILE in *.png; do
  filename=$(basename -- "$FILE")
  filename="${filename%.*}"
  echo $filename;
  #gm convert $FILE mono:- | heatshrink -w 8 -l 4 > "${filename}.hs"
  #python_string="import struct; d=open(\"${filename}.hs\", \"rb\").read(); o=open(\"${filename}.icon\", \"wb\"); o.write(struct.pack(\"<BH\", 1, len(d))); o.write(d)"
  #echo $python_string
  #python -c "$python_string"
  (echo -ne '\x00'; gm convert $FILE mono:-) > "${filename}.icon"
  rm -f "${filename}.hs"
done
