# EAs 2

Run initial experiment:

```shell
cd ../framspy
```

```shell
for F in 0 1 4 9; do
    for ((N=1; N<=3; N++)); do
        python FramsticksEvolution_logged.py -path "../Framsticks50rc30/" -sim "eval-allcriteria.sim;deterministic.sim;sample-period-2.sim;only-body.sim" -opt vertpos -max_numparts 30 -genformat "$F" -popsize 50 -generations 200 -execution_time_savefile "../EAs2/runtimes/runtime-f$F-$N.txt" -hof_size 1 -hof_savefile "../EAs2/HoF/HoF-f$F-$N.gen " -log_savefile "../EAs2/logs/log-f$F-$N.csv"
    done
done
```

Copy evolution loop with modified evaluation and run another experiment:

```shell
cp ../EAs2/FramsticksEvolution_logged_eval.py ./
```

```shell
for F in 0 1 4 9; do
    for ((N=1; N<=3; N++)); do
        python FramsticksEvolution_logged_eval.py -path "../Framsticks50rc30/" -sim "eval-allcriteria.sim;deterministic.sim;sample-period-2.sim;only-body.sim" -opt vertpos -max_numparts 30 -genformat "$F" -popsize 50 -generations 200 -execution_time_savefile "../EAs2/runtimes/runtime-f$F-$N-eval.txt" -hof_size 1 -hof_savefile "../EAs2/HoF/HoF-f$F-$N-eval.gen" -log_savefile "../EAs2/logs/log-f$F-$N-eval.csv"
    done
done
```

Visualize:

```shell
cd ../EAs2
```

```shell
python visualize_representations.py ./
```
