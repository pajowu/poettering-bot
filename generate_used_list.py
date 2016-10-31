prime = 109847
conuter = 178

with open("wordlist") as wordlistfile:
	w = wordlistfile.read().split()
	
for i in range(0,178):
	n = i*prime%len(w)
	with open("blacklist","a") as blf:
		blf.write(w[n] + "\n")