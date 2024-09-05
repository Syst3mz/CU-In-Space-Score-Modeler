A quick tool to optimize the score for CU in space's rocket. I make a number of assumptions in this program which means 
results are likely to be significantly off, but for a first pass I think these numbers are reasonable.

# How to use:
1. Grab a compiled binary (windows only for now) or build from source.
2. Run the program.

# Configuration
Configuration is handled through a file called config.json, which must be next to the program in the file system. 
If the program can't find a config.json, one will be created with the following defaults.
- `golf_ball_mass_kg`: `0.04592623`
  - Mass of a golf balls in kilograms. This value is gathered from [here](https://en.wikipedia.org/wiki/Golf_ball#:~:text=A%20golf%20ball%20is%20a,%2C%20distance%2C%20and%20symmetry%20limits.).
- `dry_mass_kg`: `4.0`
  - The mass of everything on the rocket that isn't golf balls.
- `max_impulse_ns`: `5120`
  - The maximum amount of impulse for the rocket. MUST BE AN INTEGER.
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
- `output_file`: `output.txt`
  - Where should the output file be put/called.

# Assumptions
- No air resistance / drag / atmospheric forces.
- The entire impulse of the engine is dumped into the rocket at time = 0.
- Mass does not change at any point during the flight, and the entire engine is consumed when fired.
- Adding more golf balls does not account for the increase in mass required to hold said golf balls.
- And I didn't mess up my physics...I am programmer not a physicist, please let me know if something is terribly wrong.