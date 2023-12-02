import os

f = open(f"mod.rs", "a")
for i in range(2, 26):
    dest = "day" + f"{i:02}" + ".rs"
    os.popen("copy template.rs " + dest)
    f.write("pub mod day" + f"{i:02}" + ";\n")
f.close()
