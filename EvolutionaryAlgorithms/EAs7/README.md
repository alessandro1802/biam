# EAs 7

Copy the evolution loop and parameters for running the experiment:

```shell
cp FramsticksEvolution_logged_coords.py ../framspy/
cp eval-allcriteria-mini.sim ../Framsticks50rc30/data/
```

Run experiment:

```shell
cd ../framspy
```

f9:

```shell
for popsize in 10 50 100; do
    for N in {1..5}; do
        python FramsticksEvolution_logged_coords.py \
            -path ../Framsticks50rc30/ \
            -sim "eval-allcriteria-mini.sim;deterministic.sim;sample-period-longest.sim;recording-body-coords.sim" \
            -opt vertpos \
            -max_numparts 15 \
            -max_numjoints 30 \
            -max_numneurons 20 \
            -max_numconnections 30 \
            -genformat "9" \
            -popsize $popsize \
            -generations 300 \
            -hof_size 1 \
            -tournament 7 \
            -hof_savefile "../EAs7/HoF/HoF-f9-$popsize-$N.gen" \
            -execution_time_savefile "../EAs7/runtimes/runtime-f9-$popsize-$N.txt" \
            -log_savefile "../EAs7/logs/log-f9-$popsize-$N.csv"
    done
done
```

f0, f1:

```shell
for genformat in 0 1; do
    for N in {1..5}; do
        python FramsticksEvolution_logged_coords.py \
            -path ../Framsticks50rc30/ \
            -sim "eval-allcriteria-mini.sim;deterministic.sim;sample-period-longest.sim;recording-body-coords.sim" \
            -opt vertpos \
            -max_numparts 15 \
            -max_numjoints 30 \
            -max_numneurons 20 \
            -max_numconnections 30 \
            -genformat "$genformat" \
            -popsize 50 \
            -generations 300 \
            -hof_size 1 \
            -tournament 7 \
            -hof_savefile "../EAs7/HoF/HoF-f$genformat-50-$N.gen" \
            -execution_time_savefile "../EAs7/runtimes/runtime-f$genformat-50-$N.txt" \
            -log_savefile "../EAs7/logs/log-f$genformat-50-$N.csv"
    done
done
```

Visualize:

```shell
cd ../EAs7
```

```shell
python visualize_parameters.py ./
```
