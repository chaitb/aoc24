## Day 1: Historian Hysteria

The Chief Historian is missing, and a group of Senior Historians needs your help to check historically significant locations. They believe he must be in one of the first fifty places they plan to explore. 

To gather clues, they've found two lists of location IDs that need to be reconciled to determine their similarity.

### Part One

You need to compare two lists of location IDs by pairing up their numbers. Pair the smallest from one list with the smallest from the other, continuing through to the largest. Calculate the distance between each pair and sum those distances to find the total distance between the lists.

**Example:**
```
3   4
4   3
2   5
1   3
3   9
3   3
```

The total distance for the above example is `11`.

Your actual left and right lists contain many location IDs. What is the total distance between your lists?

Your puzzle answer was `1320851`.

### Part Two

Next, determine how often each number in the left list appears in the right list. Calculate a total similarity score by adding up the products of each number in the left list and its frequency in the right list.

Using the same example:
```
3   4
4   3
2   5
1   3
3   9
3   3
```

The similarity score for this example is `31`.

What is the similarity score for your actual lists?

Your puzzle answer was `26859182`.