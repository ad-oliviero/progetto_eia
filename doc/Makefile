compile: clean documentazione grafici
documentazione:
	latexmk -pdf -quiet -xelatex -interaction=nonstopmode -file-line-error $@.tex
grafici:
	latexmk -pdf -quiet -xelatex -interaction=nonstopmode -file-line-error $@.tex
indent:
	latexindent -s -w documentazione.tex
	latexindent -s -w grafici.tex
clean:
	latexmk -c
	rm -f *.pdf

gitignore:
	find . -type f -print 2>/dev/null | grep -vE "(\.pdf|\.tex|\.bib|\.sh|\.cls|Makefile|\.gitignore)" | sed "s/.*\./*./g;s/\.\///g" > .gitignore
	cat .gitignore

