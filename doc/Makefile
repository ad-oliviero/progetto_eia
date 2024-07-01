compile: clean
	latexmk -pdf -quiet -xelatex -interaction=nonstopmode -file-line-error documentazione.tex
	latexmk -pdf -quiet -xelatex -interaction=nonstopmode -file-line-error grafici.tex
indent:
	latexindent -s -w documentazione.tex
	latexindent -s -w grafici.tex
clean:
	latexmk -c

