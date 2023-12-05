import os

# Read the code template
f_template = open("template.rs", "r")
content = f_template.read()
f_template.close()

f = open(f"mod.rs", "a")
for i in range(4, 5):
    # Add mod dayXX; to to mod.rs
    f.write("pub mod day" + f"{i:02}" + ";\n")

    # Write out the code file for the day: dayXX.rs
    dest = "day" + f"{i:02}" + ".rs"
    newContent = content.replace("_test.txt", f"{i:02}" + "_test.txt")
    f_day = open(dest, "w")
    f_day.write(newContent)
    f_day.close()
f.close()
