from morphrs_py import MorphAnalyzer

morph_analyzer = MorphAnalyzer.open("dict/")
print(morph_analyzer)
print(morph_analyzer.parse_word("стали"))
