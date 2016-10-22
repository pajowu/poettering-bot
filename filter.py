from nltk.tag import pos_tag

with open("words.txt") as srcfile:
    with open("wordlist","w") as nounfile:
        for line in srcfile:
            word = line.split("\t")[0].strip()
            tagged = pos_tag([word])
            for word,pos in tagged:
                if pos.startswith("N") and len(word) > 2 and not word.endswith("'s"):
                    nounfile.write(word + "\n")