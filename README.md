# Word Count (wc)

`wc` is a Rust clone of the common Unix command line utility used to display the number of lines, words, characters, and bytes in a text file or input stream.

## Usage
```
wc [OPTIONS] file
```

### Options
```
  -c, --bytes            print the byte counts
  -m, --chars            print the character counts
  -l, --lines            print the newline counts
  -w, --words            print the word counts
      --help             display this help and exit
```

## Examples
1. Count lines, words, and characters in a file:
```
wc filename.txt
```

2. Count the characters in multiple files:
```
wc -m file1.txt file2.txt
```

3. Count bytes in an input stream:
```
cat filename.txt | wc -c
```

## Contributing

If you find any issues or have suggestions for improvements, please feel free to open an issue or create a pull request.

