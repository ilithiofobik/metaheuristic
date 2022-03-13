import os
  
for filename in os.scandir():
	name = filename.name
	if filename.is_file() and "tour" in name:
		os.system(f"mv {name} tours/{name}")
		print(filename.name)

for filename in os.scandir():
	if filename.is_file():
		name = filename.name
		with open (name, "r") as myfile:
			data = myfile.read()
			if "EUC_2D" in data:
				os.system(f"mv {name} tours/{name}")
				print(filename.name)
			