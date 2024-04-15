from textractor import extract_text_from_file

path_to_file = "./sample_files/bitcoin.pdf"
bitcoin_text = extract_text_from_file(path_to_file)
print(bitcoin_text)

path_to_file = "./sample_files/lorem.docx"
lorem_text = extract_text_from_file(path_to_file)
print(lorem_text)