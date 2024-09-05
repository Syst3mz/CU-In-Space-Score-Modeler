A quick tool to optimize the score for CU in space's rocket. I make a number of assumptions in this program which means 
results are likely to be significantly off, but for a first pass I think these numbers are reasonable.

# How to use:
1. Grab a compiled binary (windows only for now) or build from source. Let me know if other OSes are needed.
2. Run the program.

# Interpreting Results
The program will output a list of criterion which looks like this:

```
#1: Apogee: 12191.535, Golf Balls: 13, Impulse: 292 @ 35524.625
#2: Apogee: 12183.945, Golf Balls: 11, Impulse: 247 @ 35488.04
#3: Apogee: 12155.773, Golf Balls: 14, Impulse: 314 @ 35398.785
#4: Apogee: 12142.874, Golf Balls: 12, Impulse: 269 @ 35389.61
```
The number on the left, is which solution number it is. The middle is the particular solution that was found, and the 
number following the `@` is the score based on the scoring function. If there are multiple stages, they will be listed 
by stage number in the rocket stack, along with the impulse of that stage.

# Configuration
Configuration is handled through a file called `config.json`, which must be next to the program in the file system. 
If the program can't find a `config.json`, one will be created with the following defaults.
- `golf_ball_mass_kg`: `0.04592623`
  - Mass of a golf balls in kilograms. This value is gathered from [here](https://en.wikipedia.org/wiki/Golf_ball#:~:text=A%20golf%20ball%20is%20a,%2C%20distance%2C%20and%20symmetry%20limits.).
- `stages`: `{"stage_addition_function": "Addition", "stages_dry_mass": [4.0, 2.0]}`
  - Stages object, comprised of the dry mass of each stage, and a method to combine the stages together.
    - `stage_addition_function`: `Addition`,
      - How should the dry masses of the stages be added. Valid options are `"Addition"` or `"None"` (case-sensitive). Use `"Addition"` if your stages dry mass do not include the mass of the stages before them. Use `"None"` if your stages dry mass include the stages before them.
    - `stages_dry_mass`: `[4.0]`
      - An array of the dry masses of the rocket. The left-most element in the array is the first stage, and the right most element is the final stage.
  - The mass of everything on the rocket that isn't golf balls or motors.
- `min_stage_impulse`: `0`
  - The maximum amount of impulse for the rocket. MUST BE AN INTEGER.
- `max_total_impulse`: `5120`
  - The maximum amount of impulse for the rocket. MUST BE AN INTEGER.
- `min_golf_balls`: `1`
  - The minimum number of golf balls that should be launched.
- `max_golf_balls`: `1000`
  - The maximum number of golf balls the program should search for. MUST BE AN INTEGER.
- `min_altitude_m`: `2743.2`
  - The minimum altitude in meters that a rocket must reach to count.
- `max_altitude_m`: `12192.0`
  - The maximum altitude in meters that a rocket may reach to count.
- `gravity_ms`: `9.81`
  - The force of gravity in meters / second.
- `max_tracked_solutions`: `50`
  - Keep the top N solutions, by default this is 50.
- `output_file`: `"output.txt"`
  - Where should the output file be put/called.

# Assumptions
- No air resistance / drag / atmospheric forces.
- The entire impulse of the engine is dumped into the rocket at time = 0.
- Mass does not change at any point during the flight, and the entire engine is consumed when fired.
- Adding more golf balls does not account for the increase in mass required to hold said golf balls.
- And I didn't mess up my physics...I am programmer not a physicist, please let me know if something is terribly wrong.

# Performance
Using multiple stages causes problems. This program exhaustively searches every possible combination of impulses per 
stage. This is very slow, since the number of distinct combinations grows very rapidly with the number of stages. If the
simulation gains any more fidelity I will need to switch to a better method, probably simulated annealing.