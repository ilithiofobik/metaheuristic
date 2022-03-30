def to_table(filename):
    with open(filename, 'r') as f:
        line = f.readline()
        num = line.count(";")
    with open(filename, 'r') as f:
        latex_name = "latex_" + filename
        text = f.read()
        cols = "|" + (num + 1) * "c|"
        text = text.replace(";", " & ")
        text = text.replace("\n", "\\\\\n\\hline\n")
        text = "\\begin{center}\n\\begin{tabular}{" + cols + "}\n\\hline\n" + text + "\\end{tabular}\n\\end{center}\n"
        with open(latex_name, 'w') as f1:
            f1.write(text)

if __name__ == "__main__":
    name = input()
    to_table(name)
