# Grid Snakes

During an evening, I and my sister discussed, how efficient one could implement snakes. I followed the thought and started working on a snakes based on an n times m array (the world grid). This is the result. It is reasonably optimized for constant time execution steps in the model part (drawing the world always takes O(grid size), unless you do something fancy) and has O(n * m) memory consumption.

I've added an additional `ai` module as play ground for some game agent experiments.

## Pending Tasks

- Add stones
- Add CLI argument parsing for settings
