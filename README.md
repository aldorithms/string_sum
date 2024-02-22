# Note to me:
Before doing anything, always make sure you're in the proper mode by doing:
```sh
source .env/bin/activate
```
You'll know when your terminal starts with (.env)

Compile using:
```sh 
maturin develop
```

Then test using:
```sh
python test.py
mojo test.mojo
```