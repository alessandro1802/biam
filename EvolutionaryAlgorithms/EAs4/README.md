# EAs 4

Run experiment:

```shell
cd ../framspy
```

```shell
for genformat in 0 1 4 9; do
    for N in {1..50}; do
        python FramsticksEvolution_logged.py \
            -path ../Framsticks50rc30/ \
            -sim "eval-allcriteria.sim;deterministic.sim;sample-period-longest.sim;my-own-probab-0.sim" \
            -opt vertpos \
            -max_numparts 15 \
            -max_numjoints 30 \
            -max_numneurons 20 \
            -max_numconnections 30 \
            -genformat "$genformat" \
            -popsize 50 \
            -generations 20 \
            -hof_size 200 \
            -tournament 7 \
            -hof_savefile "../EAs4/HoF/HoF-vel-probab-f$genformat-$N.gen" \
            -execution_time_savefile "../EAs4/runtimes/runtime-f$genformat-$N.txt" \
            -log_savefile "../EAs4/logs/log-f$genformat-$N.csv" &
    done
    wait
done
```

Calculate neighbours:

```shell
cp ../EAs4/generate_neighbours.py ./
```

```shell
python generate_neighbours.py \
    -path ../Framsticks50rc30/ \
    -sim "eval-allcriteria.sim;deterministic.sim;sample-period-longest.sim;my-own-probab-0.sim" \
    -opt vertpos \
    -max_numparts 15 \
    -max_numjoints 30 \
    -max_numneurons 20 \
    -max_numconnections 30 \
    -input_path "../EAs4/HoF/" \
    -output_path "../EAs4/"
```

Visualize:

```shell
cd ../EAs4
```

```shell
python visualize_representations.py ./
```
