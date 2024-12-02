## Day 2: Red-Nosed Reports

The first location The Historians want to search isn't far from the Chief Historian's office. At the Red-Nosed Reindeer nuclear fusion/fission plant, the engineers urgently need your help analyzing some unusual data (your puzzle input), which consists of reports with levels represented as numbers separated by spaces.

### Problem Description

A report is considered *safe* if:

1. The levels are either *all increasing* or *all decreasing*.
2. The difference between any two adjacent levels is *at least one* and *at most three*.

For example:

```
7 6 4 2 1  → Safe (all decreasing)
1 2 7 8 9  → Unsafe (2 to 7 increases by 5)
```

Determine how many reports are *safe* from the unusual data provided.

### Part Two

The engineers realized they forgot to mention the Problem Dampener, which allows the reactor safety systems to tolerate a *single bad level* in an otherwise safe report. Update the analysis to count a report as safe if removing one level would make it safe.

Calculate how many reports are now safe with the Problem Dampener taken into account.