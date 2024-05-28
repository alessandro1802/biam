# EAs 1
Copy mutation parameters to the configs:

```shell
cp mutation_intesities/*.sim ../Framsticks50rc30/data
cp FramsticksEvolution_logged.py ../framspy/
```

```shell
cd ../framspy
```

One run:

```shell
python FramsticksEvolution.py -path "../Framsticks50rc30/"  -sim "eval-allcriteria.sim;deterministic.sim;sample-period-2.sim" -opt vertpos -max_numparts 30 -max_numgenochars 50 -initialgenotype "/*9*/BLU" -popsize 50 -generations 20 -hof_size 1
```

Repeating for `N` runs and saving the best solution for every run with its fitenss:

```shell
for ((N=1; N<=10; N++)); do
    python FramsticksEvolution.py -path "../Framsticks50rc30/"  -sim "eval-allcriteria.sim;deterministic.sim;sample-period-2.sim" -opt vertpos -max_numparts 30 -max_numgenochars 50 -initialgenotype "/*9*/BLU" -popsize 50 -generations 20 -hof_size 1 -hof_savefile "../EAs1/HoF/HoF-f9-$N.gen"
done
```

Adding looping over previously created mutation parameters:

```shell
for M in 0 005 01 02 03 04 05; do
    for ((N=1; N<=10; N++)); do
        python FramsticksEvolution.py -path "../Framsticks50rc30/" -sim "eval-allcriteria.sim;deterministic.sim;sample-period-2.sim;f9-mut-$M.sim" -opt vertpos -max_numparts 30 -max_numgenochars 50 -initialgenotype "/*9*/BLU" -popsize 50 -generations 20 -hof_size 1 -hof_savefile "../EAs1/HoF/HoF-f9-$M-$N.gen"
	done
done
```

Recording statistics of every generation and runtimes for the experiment.

```shell
for M in 0 005 01 02 03 04 05; do
    for ((N=1; N<=10; N++)); do
        python FramsticksEvolution_logged.py -path "../Framsticks50rc30/" -sim "eval-allcriteria.sim;deterministic.sim;sample-period-2.sim;f9-mut-$M.sim" -opt vertpos -max_numparts 30 -max_numgenochars 50 -initialgenotype "/*9*/BLU" -popsize 50 -generations 100 -execution_time_savefile "../EAs1/runtimes/runtime-f9-$M-$N.txt" -hof_size 1 -hof_savefile "../EAs1/HoF/HoF-f9-$M-$N.gen" -log_savefile "../EAs1/logs/log-f9-$M-$N.csv"
    done
done
```

Visualize:

```shell
cd ../EAs1
```

```shell
python visualize_mutation.py ./
```
