# Getting started


At first you have to open your terminal\
Go to this repository:
```bash
cd <path to this diractory>
```

## Make Input file

Make an input file of unsorted numbers:
   
```bash
java -cp solution01/target/classes com.example.MakeFile <filename.txt> <file size(MiB)> <line size> 
```

e.g for 1GiB file:
```bash
java -cp solution01/target/classes com.example.MakeFile test.txt 1048 6
 ```

# Run sorting algoritm

to get the runtime of the algoritm simply wite "time" before each command.
## Run classical mergesort

```bash
java -Xmx<max_RAM_size_in_byte> -cp solution01/target/classes com.example.ClassicalMergeSort <inputfile.txt> <ouputfile.txt>
```

e.g:
```bash
java -Xmx100M -cp solution01/target/classes com.example.ClassicalMergeSort test.txt output.txt
 ```

## Run 2-way external memory mergesort

```bash
java -Xmx<max_RAM_size_in_byte> -cp solution01/target/classes com.example.EMM <inputfile.txt> <ouputfile.txt> <Block size> 
```

e.g:
```bash
java -Xmx100M -cp solution01/target/classes com.example.EMM test.txt ouput.txt 25000000
 ```

## Run Tests