# Getting started


First, you have to open your terminal\
Go to this repository:
```bash
cd <path to this diractory>
```

## Make Input file

Make an input file of unsorted numbers:
   
```bash
java -cp Exercise02/target/classes com.example.MakeFile <filename.txt> <amount of numbers>  
```
Alternatively, you can also write the amount as basis and exponent:
```bash
java -cp Exercise02/target/classes com.example.MakeFile <filename.txt> <amount basis> <amount exponent> 
```

**the program is not designed to create files bigger than 10⁹ numbers!**

e.g for a file with 10⁶ numbers:
```bash
java -cp Exercise02/target/classes com.example.MakeFile test.txt 1000000
 ```

 or:

 ```bash
java -cp Exercise02/target/classes com.example.MakeFile test.txt 10 6
 ```

# Run sorting algoritm

to get the runtime of the algorithm simply write "time" before each command.
## Classical MergeSort

```bash
java -cp Exercise02/target/classes com.example.CMergeSort <inputfile.txt> <ouputfile.txt>
```

e.g:

```bash
java -cp Exercise02/target/classes com.example.CMergeSort test.txt out.txt
```



## Parallel MergeSort

```bash
java -cp Exercise02/target/classes com.example.PMergeSort <inputfile.txt> <ouputfile.txt> <number of cores to use>
```

e.g with 4 cores:

```bash
java -cp Exercise02/target/classes com.example.PMergeSort test.txt out.txt 4
```



## Classical QuickSort

```bash
java -cp Exercise02/target/classes com.example.CQuickSort <inputfile.txt> <ouputfile.txt> 
```

e.g :

```bash
java -cp Exercise02/target/classes com.example.CQuickSort test.txt out.txt 
```



## Parallel QuickSort

```bash
java -cp Exercise02/target/classes com.example.PQuickSort <inputfile.txt> <ouputfile.txt> <number of cores to use>
```

e.g with 4 cores:

```bash
java -cp Exercise02/target/classes com.example.PQuickSort test.txt out.txt 4
```

# Experiment 1
## Preparation


Then you need to make 'experiment01.sh' executable.
```bash
chmod u+x Exercise02/experiment01.sh
```

Now you need to look up how many cores you have.
one possibility (for Linux systems) would be to run:
 ```bash
lscpu | egrep 'Model name|Socket|Thread|NUMA|CPU\(s\)'
```

## Run experiment

```bash
 ./Exercise02/experiment01.sh <number of cores> <command for algorithm you want to run> 
```
**input file name must be test.txt and leave \<number of cores to use\> out!**

\<command for the algorithm you want to run\>:
```bash
java -cp Exercise02/target/classes com.example.P<name of the algorithm> test.txt <ouputfile.txt>
```

e.g to run parallel MergeSort on with 4 available cores:
```bash
 ./Exercise02/experiment01.sh 4 java -cp Exercise02/target/classes com.example.PMergeSort test.txt out.txt
```
output:
```bash
p= 1
real 0.000000
user 1.075100
sys 0.123000
p= 2
real 0.000000
user 1.171700
sys 0.120600
p= 3
real 0.000000
user 1.362200
sys 0.132600
p= 4
real 0.000000
user 1.504300
sys 0.139700
```


# Experiment 2

## Preparation

Then you need to make 'experiment02.sh' executable.
```bash
chmod u+x Exercise02/experiment02.sh
```
## Run experiment

**running this script can use up to 7GB RAM since it's needed for 10⁹ numbers.**

**script and classes are not designed to run tests bigger than 10⁹ numbers!**

```bash
 ./Exercise02/experiment01.sh <input exponent start> <input exponent end> <command for algorithm you want to run> 
```

**input file name must be test.txt!**
e.g to run parallel MergeSort on files from 10³ to 10⁶:
```bash
 ./Exercise02/experiment01.sh 3 6 java -cp Exercise02/target/classes com.example.PMergeSort test.txt out.txt 4
```
output:
```bash
file with 10^3 numbers
real 0.10
user 0.07
sys 0.02
 
file with 10^4 numbers
real 0.11
user 0.29
sys 0.03
 
file with 10^5 numbers
real 0.20
user 0.70
sys 0.05
 
file with 10^6 numbers
real 0.63
user 1.59
sys 0.13
```



# Run Tests


Install the package '[maven](https://maven.apache.org/index.html)' to be able to run mvn commands.\
e.g. for Ubuntu:
```bash
sudo apt install maven
```

**To run the tests make sure you're in .../leraywiese/Exercise02/**\
The following part was copied from [here](https://mkyong.com/maven/how-to-run-unit-test-with-maven/).

Run all the unit test classes.
```bash
mvn test
```

 Run a single test class.
```bash
mvn -Dtest=TestApp1 test
```

Run multiple test classes.
```bash
mvn -Dtest=TestApp1,TestApp2 test
```

Run a single test method from a test class.
```bash
mvn -Dtest=TestApp1#methodname test
```

Run all test methods that match the pattern 'testHello*' from a test class.
```bash
mvn -Dtest=TestApp1#testHello* test
```

Run all test methods that match the patterns 'testHello*' and 'testMagic*' from a test class.
```bash
mvn -Dtest=TestApp1#testHello*+testMagic* test
```


