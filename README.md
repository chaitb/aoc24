# AoC24 - Advent of Code 2024 Solutions: Human vs AI

This repository compares my solutions to Advent of Code 2024 challenges with those generated by GPT-4 (OpenAI). The goal is to explore the differences in approach, efficiency, and clarity between human-written and AI-generated code.

### Project Structure

```
.
├── Cargo.lock                     # Cargo lockfile for dependencies
├── Cargo.toml                     # Rust project configuration
├── cargo                          # Build artifacts
├── run.py                         # Python script for generating AI solutions
├── src
│   ├── bin
│   │   ├── 1.rs                   # My solution for Day 1
│   │   ├── 2.rs                   # My solution for Day 2
│   │   ├── 1-ai                   # AI solution for Day 1
│   │   │   ├── analysis.md        # AI's thought process or explanation
│   │   │   ├── input.txt          # Problem input
│   │   │   ├── main.rs            # AI's Rust implementation
│   │   │   └── question.md        # Problem description
│   │   ├── 2-ai                   # AI solution for Day 2
│   │   │   ├── analysis.md
│   │   │   ├── input.txt
│   │   │   ├── main.rs
│   │   │   └── question.md
│   └── main.rs                    # Optional main entry point
```

### Using the Python Script

The Python script run.py is used to generate solutions for specific days using GPT-4.

Command:
```bash
python3 run.py <day> <year>
```

Example: To generate the AI solution for Day 2 of 2024:
```bash
python3 run.py 2 2024
```

This will create the directory src/bin/2-ai with the following files:
	•	`analysis.md`: Explanation of the AI’s approach.
	•	`input.txt`: Problem input.
	•	`main.rs`: Rust implementation of the AI-generated solution.
	•	`question.md`: Problem description.

## Running Solutions

**Run My Solution:**
For Day 2, execute:

```bash
cat src/bin/2/input.txt | cargo run --bin 2
```

**Run AI’s Solution:**
For Day 2, execute:

```bash
cat src/bin/2-ai/input.txt | cargo run --bin 2-ai
```

# Goals of the Project
- Comparison of Solutions: Understand the differences in problem-solving approaches between human and AI.
- Performance Evaluation: Analyze the efficiency of solutions in terms of runtime and memory.
- Learning and Insights: Learn new patterns or techniques from GPT-generated solutions.

Feel free to fork the repository, add your own solutions, or improve the automation script.

# Acknowledgments
- Thanks to Advent of Code for the challenges.
- Powered by GPT-4 for generating AI-based solutions.