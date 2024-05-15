# Evolutionary algorithms

## Framsticks set-up
1. Download [Framsticks50rc30](http://www.framsticks.com/apps-devel), move it here.
2. Download `framspy`:
	- install `git-svn` e.g. `brew install git-svn`
	- ```git-svn clone URL``` where URL is [this directory](https://www.framsticks.com/svn/framsticks/framspy/).
3. Virtual environment:
```shell
python3.8 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```
4. Test installation:
```shell
python framspy/frams-test.py Framsticks50rc30
cp framspy/*.sim Framsticks50rc30/data
cp run-deap-examples.sh framspy/
cd framspy && ./run-deap-examples.sh && cd ../
```
5. Run with:
```shell
wine Framsticks50rc30/Framsticks.exe
```
