# EAs 3

Copy parameters:

```shell
cp my-own-probab-* ../Framsticks50rc30/data
```

Run experiments:

```shell
cd ../framspy
```

```shell
for P in {0..2}; do
    for N in {1..5}; do
        python FramsticksEvolution_logged.py \
            -path ../Framsticks50rc30/ \
            -sim "eval-allcriteria.sim;deterministic.sim;sample-period-longest.sim;my-own-probab-$P.sim" \
            -opt velocity \
            -max_numparts 15 -max_numjoints 30 -max_numneurons 20 -max_numconnections 30 \
            -genformat 0 \
            -pxov 0 \
            -popsize 75 -generations 20 \
            -hof_size 1 -hof_savefile "../EAs3/HoF/HoF-vel-probab-$P-$N.gen" \
            -execution_time_savefile "../EAs3/runtimes/runtime-f0-$P-$N.txt" \
            -log_savefile "../EAs3/logs/log-f0-$P-$N.csv"
    done
done
```

Experiment with modification of mutation function:

```shell
cp ../EAs3/FramsticksEvolution_logged_mut.py ./
```

```shell
for P in {3..3}; do
    for N in {1..5}; do
        python FramsticksEvolution_logged_mut.py \
            -path ../Framsticks50rc30/ \
            -sim "eval-allcriteria.sim;deterministic.sim;sample-period-longest.sim;my-own-probab-$P.sim" \
            -opt velocity \
            -max_numparts 15 -max_numjoints 30 -max_numneurons 20 -max_numconnections 30 \
            -genformat 0 \
            -pxov 0 \
            -popsize 75 -generations 20 \
            -hof_size 1 -hof_savefile "../EAs3/HoF/HoF-vel-probab-$P-$N.gen" \
            -execution_time_savefile "../EAs3/runtimes/runtime-f0-$P-$N.txt" \
            -log_savefile "../EAs3/logs/log-f0-$P-$N.csv"
    done
done
```

Visualize:

```shell
cd ../EAs3
```

```shell
python visualize_parameters.py ./
```
