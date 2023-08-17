 #!/bin/bash


    n=$1; shift
    (($# > 0)) || return
    m=$1; shift
    (($# > 0)) || return                   # bail if no command given
    for ((i = n; i < m+1; i++)); do
	echo "file with 10^$i numbers";
        java -Xms7G -cp Exercise02/target/classes com.example.MakeFile test.txt 10 $i &>/dev/null;
        time -p "$@" 1>/dev/null;
	echo " "
    done
