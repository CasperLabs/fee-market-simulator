# Examples for AutoPrice

First, make sure the demand profiles are generated:

```
bash ../generate_demand_profiles.sh
```

## Short-term simulation of the daily demand cycle

```
cargo run --release -- -c short-term-daily-cycle/config.toml
```

## Simulation of the daily demand cycle superimposed with a longer-term trend

```
cargo run --release -- -c long-term-trend/config.toml
```

