MAIN = main
OUTPUT_DIR = output

all: compile bibtex compile twice

compile:
	latexmk -pdflua -output-directory=$(OUTPUT_DIR) $(MAIN).tex

bibtex:
	@if grep -q '\\citation' $(OUTPUT_DIR)/$(MAIN).aux; then \
		bibtex $(OUTPUT_DIR)/$(MAIN); \
	else \
		echo "No citations found in the document."; \
	fi

twice:
	latexmk -pdflua -output-directory=$(OUTPUT_DIR) $(MAIN).tex
	latexmk -pdflua -output-directory=$(OUTPUT_DIR) $(MAIN).tex

clean:
	rm -f $(OUTPUT_DIR)/*.aux $(OUTPUT_DIR)/*.log $(OUTPUT_DIR)/*.bbl \
	      $(OUTPUT_DIR)/*.blg $(OUTPUT_DIR)/*.toc $(OUTPUT_DIR)/*.out \
	      $(OUTPUT_DIR)/*.lof $(OUTPUT_DIR)/*.lot $(OUTPUT_DIR)/*.synctex.gz
