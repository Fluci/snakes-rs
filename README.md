# Grid Snakes

During an evening, I and my sister discussed, how efficient one could implement snakes. I followed the thought and started working on a snakes based on an n times m array (the world grid). This is the result. It is reasonably optimized for constant time execution steps in the model part (drawing the world always takes O(grid size), unless you do something fancy) and has O(n * m) memory consumption.

I've added an additional `ai` module as play ground for some game agent experiments.

## How to run

To run in single player mode:

```bash
cargo run --bin play
```

Run for two players:

```bash
cargo run --bin play -- -m
```
(Controls: arrow keys and wasd)

Run agent:

```bash
cargo run --bin learn
```

## Pending Tasks

- Add CLI argument parsing for settings
