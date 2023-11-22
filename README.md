# DAILY WORKOUT PLAN

Daily workoup plan is a canister that generates user's workout plan based on their profile.

The canister can be tested locally as defined in the instructions below.

## Built With

    - azle
    - dfx
    - Rust
    - Ic protocol
    - node

## Getting Started

    To get a local copy up and running follow these simple steps.

## Install

1. Clone the repository to your local machine

```sh
$ git clone git@github.com:wathigo/DailyWorkoutPlan.git
```

2. cd into the directory

```sh
$ cd DailyWorkoutPlan
```

3. install dependencies

```sh
npm install
```

Initialize the local Internet Computer

```sh
dfx start --background
```

Register, build, and deploy canister on the local Internet Computer

```sh
npm run gen-deploy
```

## Usage

After deploying the program, one can use the candind interface or the command-line to interact with the canister endpoints.

## Authors

**Simon Wathigo**

- Github: [@wathigo](https://github.com/wathigo)

## Contributing

    Contributions, issues and feature requests are welcome!

Feel free to check the [issues page](../../issues).
