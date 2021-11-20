from easygui import *
import sys
# 1st arg, title of thw window
# 2nd arg, prompt message
# 3rd arg, choices

# Window title
title = str(sys.argv[1])
prompt = str(sys.argv[2])
# argument will be a semicolon delimited list, like "1;2;3;"
choices = sys.argv[3].split(";")
# Create window
output = multchoicebox(prompt, title, choices)
# convert back to semicolon delimited list
print(str(";".join(list(map(str, output)))))