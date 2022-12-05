# Time Capsule

## Proof of Concept
The idea for this project is a very basic "time capsule" contract. While there are many uses for a time capsule, one use would be to allow users to privately make a prediction that could be publicly verified after enough time has passed. E.g. Bob and Alice each have differing opinions of which video game will win Game of the Year. They each deploy and invoke the Time Capsule contract to store their prediction, which cannot be modified and will not be accessible via the `get` function until the agreed upon amount of time has passed.

### Limitations and Future Work
Data stored in env.data() is publicly accessible on the ledger. Work needs to be done to enable encryption so that data remains private until the target timestamp has passed.

## Usage

### init
This must be called first - it takes two args. Example:
```soroban invoke --id $id --fn init --arg 100 --arg 41b76af9920cd2f8a7050db947eb609f1314e3e303c05735bb8969033ca15839```
This sets up the script to require a delay of 100 seconds to retrieve the string "testing this string".

### check
```soroban invoke --id $id --fn check```
Check to see how long remains before you can retrieve data. Returns an error if `init` has not been run.

### get
```soroban invoke --id $id --fn get```
Get the data. Returns an error if `init` has not been run, or if not enough time has elapsed.