#!/bin/bash

avg_time() {
    n=$1; shift
    (($# > 0)) || return                   # bail if no command given
    for ((i = 0; i < n; i++)); do
	java -cp Exercise02/target/classes com.example.MakeFile test.txt 10 6;
        { time -p "$@" &>/dev/null; } 2>&1 # ignore the output of the command
                                           # but collect time's output in stdout
    done | awk '
        /real/ { real = real + $2; nr++ }
        /user/ { user = user + $2; nu++ }
        /sys/  { sys  = sys  + $2; ns++}
        END    {
                 if (nr>0) printf("real %f\n", real/nr);
                 if (nu>0) printf("user %f\n", user/nu);
                 if (ns>0) printf("sys %f\n",  sys/ns)
               }'
}

m=$1; shift
(($# > 0)) || return                   # bail if no command given
for (( j=1; j <= m; ++j ));do
   echo "p= $j";
   avg_time 100 $@ $j; 
done
 
